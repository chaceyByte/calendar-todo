use std::sync::Arc;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Database {
    pool: Arc<SqlitePool>,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("数据库连接错误: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("SQL执行错误: {0}")]
    SqlError(sqlx::Error),
    
    #[error("数据转换错误: {0}")]
    DataError(String),
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self, DatabaseError> {
        // 如果路径是相对路径，转换为绝对路径
        let connection_string = if db_path.starts_with("/") {
            format!("sqlite://{}?mode=rwc", db_path)
        } else {
            format!("sqlite:{}?mode=rwc", db_path)
        };
        
        println!("连接数据库: {}", connection_string);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .map_err(|e| {
                eprintln!("数据库连接失败: {}", e);
                DatabaseError::ConnectionError(e)
            })?;
        
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
    
    pub async fn initialize_database(&self) -> Result<(), DatabaseError> {
        println!("初始化数据库表结构...");
        
        let sql = include_str!("../../migrations/001_initial_schema.sql");
        
        self.execute_sql_script(sql).await?;
        
        self.run_migration_002().await?;
        
        println!("数据库初始化完成");
        Ok(())
    }
    
    async fn execute_sql_script(&self, sql: &str) -> Result<(), DatabaseError> {
        let mut in_trigger = false;
        let mut current_stmt = String::new();
        
        for (line_num, line) in sql.lines().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.to_uppercase().starts_with("CREATE TRIGGER")
                || (trimmed.to_uppercase().starts_with("CREATE ") && trimmed.to_uppercase().contains(" TRIGGER "))
            {
                in_trigger = true;
                current_stmt.push_str(line);
                current_stmt.push('\n');
                continue;
            }
            
            if in_trigger {
                current_stmt.push_str(line);
                current_stmt.push('\n');
                
                let upper = trimmed.to_uppercase();
                if upper == "END;" || upper.ends_with("END;") || upper == "END" {
                    in_trigger = false;
                    let stmt = current_stmt.trim();
                    if !stmt.is_empty() {
                        println!("执行触发器 [行 {}]: {}", line_num, &stmt[..std::cmp::min(50, stmt.len())]);
                        if let Err(e) = sqlx::query(stmt).execute(&*self.pool).await {
                            eprintln!("SQL 错误 [行 {}]: {}\n完整语句:\n{}", line_num, e, stmt);
                            return Err(DatabaseError::SqlError(e));
                        }
                    }
                    current_stmt.clear();
                }
                continue;
            }
            
            if line.trim().ends_with(';') {
                current_stmt.push_str(line);
                let stmt = current_stmt.trim();
                if !stmt.is_empty() {
                    println!("执行 SQL [行 {}]: {}", line_num, &stmt[..std::cmp::min(80, stmt.len())]);
                    if let Err(e) = sqlx::query(stmt).execute(&*self.pool).await {
                        eprintln!("SQL 错误 [行 {}]: {}\n完整语句:\n{}", line_num, e, stmt);
                        return Err(DatabaseError::SqlError(e));
                    }
                }
                current_stmt.clear();
            } else if !trimmed.is_empty() && !trimmed.starts_with("--") {
                current_stmt.push_str(line);
                current_stmt.push(' ');
            }
        }
        
        Ok(())
    }
    
    async fn run_migration_002(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 is_important 字段是否存在）
        let column_exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM pragma_table_info('tasks') WHERE name = 'is_important'"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if column_exists.is_none() {
            println!("执行数据库迁移 002: 添加 is_important 和 is_urgent 字段...");
            
            // 添加新字段
            sqlx::query("ALTER TABLE tasks ADD COLUMN is_important INTEGER DEFAULT 0")
                .execute(&*self.pool)
                .await
                .map_err(DatabaseError::SqlError)?;
            
            sqlx::query("ALTER TABLE tasks ADD COLUMN is_urgent INTEGER DEFAULT 0")
                .execute(&*self.pool)
                .await
                .map_err(DatabaseError::SqlError)?;
            
            // 根据现有 priority 字段迁移数据
            sqlx::query(
                "UPDATE tasks SET 
                    is_important = CASE 
                        WHEN priority = 'high' OR priority = 'medium' THEN 1 
                        ELSE 0 
                    END,
                    is_urgent = CASE 
                        WHEN priority = 'high' THEN 1 
                        ELSE 0 
                    END"
            )
            .execute(&*self.pool)
            .await
            .map_err(DatabaseError::SqlError)?;
            
            // 创建索引
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_important ON tasks(is_important)")
                .execute(&*self.pool)
                .await
                .map_err(DatabaseError::SqlError)?;
            
            sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_urgent ON tasks(is_urgent)")
                .execute(&*self.pool)
                .await
                .map_err(DatabaseError::SqlError)?;
            
            println!("数据库迁移 002 完成");
        }
        
        Ok(())
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}