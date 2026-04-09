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
        
        // 创建所有必要的表
        let sql = include_str!("../../migrations/001_initial_schema.sql");
        
        // 分割 SQL 语句并逐条执行
        for statement in sql.split(";") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt)
                    .execute(&*self.pool)
                    .await
                    .map_err(DatabaseError::SqlError)?;
            }
        }
        
        // 执行迁移：添加 is_important 和 is_urgent 字段
        self.run_migration_002().await?;
        
        // 执行迁移：优化节假日和补班表结构
        self.run_migration_003().await?;
        
        // 执行迁移：添加任务工作时长记录功能
        self.run_migration_004().await?;
        
        // 执行迁移：修复工作时长字段类型问题
        self.run_migration_005().await?;
        
        // 执行迁移：修复 work_days 字段类型问题
        self.run_migration_006().await?;
        
        // 执行迁移：添加工作时长配置表
        self.run_migration_007().await?;
        
        println!("数据库初始化完成");
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
    
    async fn run_migration_003(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 holiday_configs 表是否有 name 字段）
        let column_exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM pragma_table_info('holiday_configs') WHERE name = 'name'"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if column_exists.is_none() {
            println!("执行数据库迁移 003: 优化节假日和补班表结构...");
            
            // 执行迁移脚本
            let sql = include_str!("../../migrations/003_holiday_makeup_enhancement.sql");
            
            // 分割 SQL 语句并逐条执行
            for statement in sql.split(";") {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&*self.pool)
                        .await
                        .map_err(DatabaseError::SqlError)?;
                }
            }
            
            println!("数据库迁移 003 完成");
        }
        
        Ok(())
    }
    
    async fn run_migration_004(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 tasks 表是否有 total_work_duration_minutes 字段）
        let column_exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM pragma_table_info('tasks') WHERE name = 'total_work_duration_minutes'"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if column_exists.is_none() {
            println!("执行数据库迁移 004: 添加任务工作时长记录功能...");
            
            // 执行迁移脚本
            let sql = include_str!("../../migrations/004_add_task_work_duration.sql");
            
            // 分割 SQL 语句并逐条执行
            for statement in sql.split(";") {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&*self.pool)
                        .await
                        .map_err(DatabaseError::SqlError)?;
                }
            }
            
            println!("数据库迁移 004 完成");
        }
        
        Ok(())
    }
    
    async fn run_migration_005(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 work_days 字段是否存在类型问题）
        // 这里我们通过检查是否有任务的工作时长需要重新计算来判断
        let needs_migration: bool = sqlx::query_scalar(
            "SELECT COUNT(*) > 0 FROM tasks WHERE total_work_duration_minutes IS NULL"
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if needs_migration {
            println!("执行数据库迁移 005: 修复工作时长字段类型问题...");
            
            // 执行迁移脚本
            let sql = include_str!("../../migrations/005_fix_work_duration_type.sql");
            
            // 分割 SQL 语句并逐条执行
            for statement in sql.split(";") {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&*self.pool)
                        .await
                        .map_err(DatabaseError::SqlError)?;
                }
            }
            
            println!("数据库迁移 005 完成");
        }
        
        Ok(())
    }

    async fn run_migration_006(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 work_days_minutes 列是否存在）
        let column_exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM pragma_table_info('task_work_records') WHERE name = 'work_days_minutes'"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if column_exists.is_none() {
            println!("执行数据库迁移 006: 修复 work_days 字段类型问题...");
            
            // 执行迁移脚本
            let sql = include_str!("../../migrations/006_fix_work_days_column.sql");
            
            // 分割 SQL 语句并逐条执行
            for statement in sql.split(";") {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&*self.pool)
                        .await
                        .map_err(DatabaseError::SqlError)?;
                }
            }
            
            println!("数据库迁移 006 完成");
        }
        
        Ok(())
    }

    async fn run_migration_007(&self) -> Result<(), DatabaseError> {
        // 检查是否需要执行迁移（检查 work_hours_config 表是否存在）
        let table_exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'work_hours_config'"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(DatabaseError::SqlError)?;
        
        if table_exists.is_none() {
            println!("执行数据库迁移 007: 添加工作时长配置表...");
            
            // 执行迁移脚本
            let sql = include_str!("../../migrations/007_add_work_hours_config.sql");
            
            // 分割 SQL 语句并逐条执行
            for statement in sql.split(";") {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&*self.pool)
                        .await
                        .map_err(DatabaseError::SqlError)?;
                }
            }
            
            println!("数据库迁移 007 完成");
        }
        
        Ok(())
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}