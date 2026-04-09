use crate::database::{Database, DatabaseError};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

/// 数据库管理器错误类型
#[derive(Error, Debug, Clone)]
pub enum DbManagerError {
    #[error("数据库正在切换中，请稍后重试")]
    Switching,
    
    #[error("数据库未初始化")]
    NotInitialized,
    
    #[error("数据库错误: {0}")]
    Database(String),
    
    #[error("切换超时")]
    SwitchTimeout,
    
    #[error("路径验证失败: {0}")]
    PathValidation(String),
}

impl From<DatabaseError> for DbManagerError {
    fn from(e: DatabaseError) -> Self {
        DbManagerError::Database(e.to_string())
    }
}

/// 数据库状态
#[derive(Debug, Clone)]
pub enum DatabaseState {
    /// 已就绪，可以正常使用
    Ready(Arc<Database>),
    /// 正在切换中
    Switching,
    /// 未初始化
    Uninitialized,
    /// 错误状态
    Error(String),
}

/// 数据库管理器
/// 
/// 提供线程安全的数据库访问和动态切换功能
pub struct DatabaseManager {
    /// 当前数据库状态
    state: Arc<RwLock<DatabaseState>>,
    /// 切换超时时间（秒）
    switch_timeout_secs: u64,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(DatabaseState::Uninitialized)),
            switch_timeout_secs: 30,
        }
    }

    /// 初始化数据库
    pub async fn initialize(&self, db_path: &str) -> Result<(), DbManagerError> {
        let mut state = self.state.write().await;
        
        // 验证路径
        self.validate_path(db_path)?;
        
        // 创建数据库连接
        let db = Database::new(db_path).await?;
        db.initialize_database().await?;
        
        *state = DatabaseState::Ready(Arc::new(db));
        
        println!("✅ 数据库管理器初始化成功: {}", db_path);
        Ok(())
    }

    /// 获取当前数据库状态
    pub async fn get_state(&self) -> DatabaseState {
        self.state.read().await.clone()
    }

    /// 检查数据库是否就绪
    pub async fn is_ready(&self) -> bool {
        matches!(self.state.read().await.clone(), DatabaseState::Ready(_))
    }

    /// 执行数据库操作
    /// 
    /// 如果数据库正在切换或未初始化，会返回错误
    pub async fn with_db<F, T>(&self, operation: F) -> Result<T, DbManagerError>
    where
        F: FnOnce(&Database) -> Result<T, DatabaseError>,
    {
        let state = self.state.read().await;
        
        match &*state {
            DatabaseState::Ready(db) => {
                operation(db).map_err(|e| e.into())
            }
            DatabaseState::Switching => {
                Err(DbManagerError::Switching)
            }
            DatabaseState::Uninitialized => {
                Err(DbManagerError::NotInitialized)
            }
            DatabaseState::Error(msg) => {
                Err(DbManagerError::Database(msg.clone()))
            }
        }
    }

    /// 执行异步数据库操作
    pub async fn with_db_async<F, Fut, T>(&self, operation: F) -> Result<T, DbManagerError>
    where
        F: FnOnce(Arc<Database>) -> Fut,
        Fut: std::future::Future<Output = Result<T, DatabaseError>>,
    {
        let state = self.state.read().await;
        
        match &*state {
            DatabaseState::Ready(db) => {
                operation(db.clone()).await.map_err(|e| e.into())
            }
            DatabaseState::Switching => {
                Err(DbManagerError::Switching)
            }
            DatabaseState::Uninitialized => {
                Err(DbManagerError::NotInitialized)
            }
            DatabaseState::Error(msg) => {
                Err(DbManagerError::Database(msg.clone()))
            }
        }
    }

    /// 切换数据库
    /// 
    /// 这是一个原子操作，会阻塞新的数据库请求直到切换完成
    pub async fn switch_database(&self, new_path: &str) -> Result<(), DbManagerError> {
        println!("🔄 开始切换数据库到: {}", new_path);
        
        // 1. 验证新路径
        self.validate_path(new_path)?;
        
        // 2. 设置为切换状态（获取写锁）
        {
            let mut state = self.state.write().await;
            match &*state {
                DatabaseState::Switching => {
                    return Err(DbManagerError::Switching);
                }
                _ => {
                    *state = DatabaseState::Switching;
                }
            }
        }
        
        println!("⏳ 等待现有操作完成...");
        
        // 3. 给现有操作一点时间完成（通过短暂等待）
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // 4. 创建新数据库连接
        println!("📝 创建新数据库连接...");
        let new_db = match Database::new(new_path).await {
            Ok(db) => db,
            Err(e) => {
                // 切换失败，恢复到错误状态
                let mut state = self.state.write().await;
                *state = DatabaseState::Error(format!("切换失败: {}", e));
                return Err(DbManagerError::Database(e.to_string()));
            }
        };
        
        // 5. 初始化新数据库
        println!("🔧 初始化新数据库...");
        if let Err(e) = new_db.initialize_database().await {
            let mut state = self.state.write().await;
            *state = DatabaseState::Error(format!("初始化失败: {}", e));
            return Err(DbManagerError::Database(e.to_string()));
        }
        
        // 6. 更新状态为新数据库
        {
            let mut state = self.state.write().await;
            *state = DatabaseState::Ready(Arc::new(new_db));
        }
        
        println!("✅ 数据库切换成功: {}", new_path);
        Ok(())
    }

    /// 验证数据库路径
    fn validate_path(&self, path: &str) -> Result<(), DbManagerError> {
        if path.is_empty() {
            return Err(DbManagerError::PathValidation(
                "数据库路径不能为空".to_string()
            ));
        }
        
        let path_buf = std::path::PathBuf::from(path);
        
        // 检查父目录是否存在
        if let Some(parent) = path_buf.parent() {
            if !parent.exists() {
                return Err(DbManagerError::PathValidation(
                    format!("父目录不存在: {:?}", parent)
                ));
            }
            
            // 检查是否有写入权限（尝试创建一个临时文件）
            let test_file = parent.join(".write_test");
            match std::fs::File::create(&test_file) {
                Ok(_) => {
                    let _ = std::fs::remove_file(&test_file);
                }
                Err(e) => {
                    return Err(DbManagerError::PathValidation(
                        format!("目录无写入权限: {}", e)
                    ));
                }
            }
        }
        
        Ok(())
    }

    /// 获取当前数据库信息
    pub async fn get_info(&self) -> Result<DatabaseInfo, DbManagerError> {
        self.with_db(|db| {
            Ok(DatabaseInfo {
                connected: true,
                pool_size: 5, // sqlx 默认连接池大小
            })
        }).await
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据库信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct DatabaseInfo {
    pub connected: bool,
    pub pool_size: u32,
}

/// 便捷宏：包装数据库操作
/// 
/// 使用示例：
/// ```rust
/// db_operation!(db_manager, |db| {
///     // 使用 db 执行操作
///     db.get_pool().fetch_one(...).await
/// })
/// ```
#[macro_export]
macro_rules! db_operation {
    ($manager:expr, $op:expr) => {
        $manager.with_db(|db| {
            $op(db)
        }).await
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_new() {
        let manager = DatabaseManager::new();
        assert!(!manager.is_ready().await);
        
        match manager.get_state().await {
            DatabaseState::Uninitialized => {}
            _ => panic!("新管理器应该是未初始化状态"),
        }
    }

    #[tokio::test]
    async fn test_validate_path() {
        let manager = DatabaseManager::new();
        
        // 空路径
        assert!(manager.validate_path("").is_err());
        
        // 不存在的目录
        assert!(manager.validate_path("/nonexistent/dir/db.sqlite").is_err());
        
        // 有效路径（临时目录）
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test.db");
        assert!(manager.validate_path(test_path.to_str().unwrap()).is_ok());
    }
}