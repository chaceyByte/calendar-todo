use std::fmt;
use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub quadrant: Quadrant,
    pub status: TaskStatus,
    pub progress: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub start_at: Option<DateTime<Utc>>,
    pub due_at: Option<DateTime<Utc>>,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived: bool,
    pub archived_at: Option<DateTime<Utc>>,
    pub tags: Vec<TagRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Quadrant {
    ImportantUrgent = 1,
    ImportantNotUrgent = 2,
    NotImportantUrgent = 3,
    NotImportantNotUrgent = 4,
}

impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quadrant::ImportantUrgent => write!(f, "重要紧急"),
            Quadrant::ImportantNotUrgent => write!(f, "重要不紧急"),
            Quadrant::NotImportantUrgent => write!(f, "紧急不重要"),
            Quadrant::NotImportantNotUrgent => write!(f, "不紧急不重要"),
        }
    }
}

impl From<i32> for Quadrant {
    fn from(value: i32) -> Self {
        match value {
            1 => Quadrant::ImportantUrgent,
            2 => Quadrant::ImportantNotUrgent,
            3 => Quadrant::NotImportantUrgent,
            4 => Quadrant::NotImportantNotUrgent,
            _ => Quadrant::ImportantUrgent,
        }
    }
}

impl From<Quadrant> for i32 {
    fn from(value: Quadrant) -> Self {
        match value {
            Quadrant::ImportantUrgent => 1,
            Quadrant::ImportantNotUrgent => 2,
            Quadrant::NotImportantUrgent => 3,
            Quadrant::NotImportantNotUrgent => 4,
        }
    }
}

impl Quadrant {
    pub fn from_importance_urgency(is_important: bool, is_urgent: bool) -> Self {
        match (is_important, is_urgent) {
            (true, true) => Quadrant::ImportantUrgent,
            (true, false) => Quadrant::ImportantNotUrgent,
            (false, true) => Quadrant::NotImportantUrgent,
            (false, false) => Quadrant::NotImportantNotUrgent,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Planning = 0,
    InProgress = 1,
    Paused = 2,
    Completed = 3,
    Archived = 4,
}

impl From<i32> for TaskStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => TaskStatus::Planning,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::Paused,
            3 => TaskStatus::Completed,
            4 => TaskStatus::Archived,
            _ => TaskStatus::Planning,
        }
    }
}

impl From<TaskStatus> for i32 {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Planning => 0,
            TaskStatus::InProgress => 1,
            TaskStatus::Paused => 2,
            TaskStatus::Completed => 3,
            TaskStatus::Archived => 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub is_important: bool,
    pub is_urgent: bool,
    pub start_at: Option<String>,
    pub due_at: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub quadrant: Option<Quadrant>,
    pub status: Option<TaskStatus>,
    pub progress: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
    pub start_at: Option<String>,
    pub due_at: Option<String>,
    pub archived: Option<bool>,
    pub add_tag_ids: Option<Vec<i64>>,
    pub remove_tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TagRef {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, FromRow)]
struct TaskRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub quadrant: i32,
    pub status: i32,
    pub progress: i32,
    pub is_important: i32,
    pub is_urgent: i32,
    pub start_at: Option<NaiveDateTime>,
    pub due_at: Option<NaiveDateTime>,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: i32,
    pub archived_at: Option<NaiveDateTime>,
}

impl Task {
    async fn from_row(row: TaskRow, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        // 获取任务的标签
        let tags = if row.id > 0 {
            sqlx::query_as::<_, TagRef>(
                "SELECT t.id, t.name, t.color FROM tags t
                INNER JOIN task_tags tt ON t.id = tt.tag_id
                WHERE tt.task_id = ?"
            )
            .bind(row.id)
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        } else {
            Vec::new()
        };
        
        Ok(Self {
            id: row.id,
            title: row.title,
            description: row.description,
            quadrant: Quadrant::from(row.quadrant),
            status: TaskStatus::from(row.status),
            progress: row.progress,
            is_important: row.is_important == 1,
            is_urgent: row.is_urgent == 1,
            start_at: row.start_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            due_at: row.due_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            user_id: row.user_id,
            created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            archived: row.archived == 1,
            archived_at: row.archived_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            tags,
        })
    }
}

pub struct TaskService {
    pool: SqlitePool,
}

impl TaskService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create_task(&self, req: CreateTaskRequest) -> Result<Task, sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        // 根据重要性和紧急性计算象限
        let quadrant = Quadrant::from_importance_urgency(req.is_important, req.is_urgent);
        
        // 插入任务
        let task_id = sqlx::query(
            "INSERT INTO tasks (title, description, quadrant, status, is_important, is_urgent, start_at, due_at, user_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"
        )
        .bind(&req.title)
        .bind(req.description.as_deref())
        .bind(i32::from(quadrant) as i64)
        .bind(i32::from(req.status.clone()) as i64)
        .bind(if req.is_important { 1 } else { 0 })
        .bind(if req.is_urgent { 1 } else { 0 })
        .bind(req.start_at.as_deref())
        .bind(req.due_at.as_deref())
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        // 添加标签
        if let Some(tag_ids) = req.tag_ids {
            for tag_id in tag_ids {
                sqlx::query(
                    "INSERT OR IGNORE INTO task_tags (task_id, tag_id, created_at) VALUES (?, ?, ?)"
                )
                .bind(task_id)
                .bind(tag_id)
                .bind(now)
                .execute(&self.pool)
                .await?;
            }
        }
        
        // 获取完整的任务对象
        self.get_task(task_id).await
    }
    
    pub async fn update_task(&self, req: UpdateTaskRequest) -> Result<Task, sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        // 构建更新语句
        sqlx::query(
            "UPDATE tasks SET updated_at = ? WHERE id = ?"
        )
        .bind(now)
        .bind(req.id)
        .execute(&self.pool)
        .await?;
        
        // 更新时间相关逻辑
        if let Some(title) = req.title {
            sqlx::query("UPDATE tasks SET title = ? WHERE id = ?")
                .bind(title)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(description) = req.description {
            sqlx::query("UPDATE tasks SET description = ? WHERE id = ?")
                .bind(description)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        // 先处理 archived 字段，因为它可能影响 status
        if let Some(archived) = req.archived {
            let archived_value = if archived { 1 } else { 0 };
            
            // 如果归档，强制设置状态为 archived (4)
            // 如果取消归档且没有指定新状态，默认设置为 planning (0)
            let has_explicit_status = req.status.is_some();
            let status_value = if archived { 
                4i64 // Archived
            } else if !has_explicit_status {
                0i64 // Planning (默认)
            } else {
                // 有指定状态，在后面处理
                -1i64
            };
            
            if status_value >= 0 {
                sqlx::query("UPDATE tasks SET archived = ?, archived_at = ?, status = ? WHERE id = ?")
                    .bind(archived_value)
                    .bind(if archived { Some(now) } else { None })
                    .bind(status_value)
                    .bind(req.id)
                    .execute(&self.pool)
                    .await?;
            } else {
                // 只更新 archived 字段，不更新状态
                sqlx::query("UPDATE tasks SET archived = ?, archived_at = ? WHERE id = ?")
                    .bind(archived_value)
                    .bind(if archived { Some(now) } else { None })
                    .bind(req.id)
                    .execute(&self.pool)
                    .await?;
            }
        }
        
        // 处理 status 字段（如果指定了且没有被 archived 处理过）
        if let Some(status) = req.status {
            // 只有在不是归档操作时才更新状态（避免覆盖 archived 设置的状态）
            if req.archived.is_none() || !req.archived.unwrap() {
                sqlx::query("UPDATE tasks SET status = ? WHERE id = ?")
                    .bind(i32::from(status) as i64)
                    .bind(req.id)
                    .execute(&self.pool)
                    .await?;
            }
        }
        
        // 更新重要性和紧急性
        if let Some(is_important) = req.is_important {
            sqlx::query("UPDATE tasks SET is_important = ? WHERE id = ?")
                .bind(if is_important { 1 } else { 0 })
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(is_urgent) = req.is_urgent {
            sqlx::query("UPDATE tasks SET is_urgent = ? WHERE id = ?")
                .bind(if is_urgent { 1 } else { 0 })
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        // 如果重要性和紧急性都更新了，重新计算象限
        if req.is_important.is_some() || req.is_urgent.is_some() {
            // 获取当前的重要性和紧急性值
            let current: (i32, i32) = sqlx::query_as(
                "SELECT is_important, is_urgent FROM tasks WHERE id = ?"
            )
            .bind(req.id)
            .fetch_one(&self.pool)
            .await?;
            
            let quadrant = Quadrant::from_importance_urgency(current.0 == 1, current.1 == 1);
            sqlx::query("UPDATE tasks SET quadrant = ? WHERE id = ?")
                .bind(i32::from(quadrant) as i64)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(quadrant) = req.quadrant {
            sqlx::query("UPDATE tasks SET quadrant = ? WHERE id = ?")
                .bind(i32::from(quadrant) as i64)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(progress) = req.progress {
            sqlx::query("UPDATE tasks SET progress = ? WHERE id = ?")
                .bind(progress)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(start_at) = req.start_at {
            sqlx::query("UPDATE tasks SET start_at = ? WHERE id = ?")
                .bind(start_at)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(due_at) = req.due_at {
            sqlx::query("UPDATE tasks SET due_at = ? WHERE id = ?")
                .bind(due_at)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }
        
        // 处理标签
        if let Some(add_ids) = req.add_tag_ids {
            for tag_id in add_ids {
                sqlx::query(
                    "INSERT OR IGNORE INTO task_tags (task_id, tag_id, created_at) VALUES (?, ?, ?)"
                )
                .bind(req.id)
                .bind(tag_id)
                .bind(now)
                .execute(&self.pool)
                .await?;
            }
        }
        
        if let Some(remove_ids) = req.remove_tag_ids {
            for tag_id in remove_ids {
                sqlx::query("DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?")
                    .bind(req.id)
                    .bind(tag_id)
                    .execute(&self.pool)
                    .await?;
            }
        }
        
        self.get_task(req.id).await
    }
    
    pub async fn delete_task(&self, task_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(task_id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    pub async fn get_task(&self, task_id: i64) -> Result<Task, sqlx::Error> {
        let row: TaskRow = sqlx::query_as(
            "SELECT * FROM tasks WHERE id = ?"
        )
        .bind(task_id)
        .fetch_one(&self.pool)
        .await?;
        
        Task::from_row(row, &self.pool).await
    }
    
    pub async fn get_tasks_by_quadrant(&self, quadrant: Quadrant, include_archived: bool) -> Result<Vec<Task>, sqlx::Error> {
        let query = if include_archived {
            "SELECT * FROM tasks WHERE quadrant = ? ORDER BY created_at DESC"
        } else {
            "SELECT * FROM tasks WHERE quadrant = ? AND archived = 0 ORDER BY created_at DESC"
        };
        
        let rows: Vec<TaskRow> = sqlx::query_as(query)
            .bind(i32::from(quadrant) as i64)
            .fetch_all(&self.pool)
            .await?;
        
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task::from_row(row, &self.pool).await?);
        }
        
        Ok(tasks)
    }
    
    pub async fn get_recent_tasks(&self, limit: i32) -> Result<Vec<Task>, sqlx::Error> {
        let rows: Vec<TaskRow> = sqlx::query_as(
            "SELECT * FROM tasks WHERE archived = 0 ORDER BY updated_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task::from_row(row, &self.pool).await?);
        }
        
        Ok(tasks)
    }
    
    pub async fn search_tasks(&self, query: &str) -> Result<Vec<Task>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);
        
        let rows: Vec<TaskRow> = sqlx::query_as(
            "SELECT * FROM tasks 
             WHERE archived = 0 
               AND (title LIKE ? OR description LIKE ?)
             ORDER BY updated_at DESC"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task::from_row(row, &self.pool).await?);
        }
        
        Ok(tasks)
    }
    
    pub async fn update_task_status(&self, task_id: i64, status: TaskStatus) -> Result<Task, sqlx::Error> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            "UPDATE tasks SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(i32::from(status.clone()) as i64)
        .bind(now)
        .bind(task_id)
        .execute(&self.pool)
        .await?;

        self.get_task(task_id).await
    }

    pub async fn get_tasks_by_status(&self, status: TaskStatus, include_archived: bool) -> Result<Vec<Task>, sqlx::Error> {
        let query = if include_archived {
            "SELECT * FROM tasks WHERE status = ? ORDER BY created_at DESC"
        } else {
            "SELECT * FROM tasks WHERE status = ? AND archived = 0 ORDER BY created_at DESC"
        };

        let rows: Vec<TaskRow> = sqlx::query_as(query)
            .bind(i32::from(status) as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task::from_row(row, &self.pool).await?);
        }

        Ok(tasks)
    }

    pub async fn get_all_tasks(&self, include_archived: bool) -> Result<Vec<Task>, sqlx::Error> {
        let query = if include_archived {
            "SELECT * FROM tasks ORDER BY created_at DESC"
        } else {
            "SELECT * FROM tasks WHERE archived = 0 ORDER BY created_at DESC"
        };

        let rows: Vec<TaskRow> = sqlx::query_as(query)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task::from_row(row, &self.pool).await?);
        }

        Ok(tasks)
    }
}

#[tauri::command]
pub async fn create_task(
    db: tauri::State<'_, Arc<Database>>,
    request: CreateTaskRequest,
) -> Result<Task, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    service.create_task(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task(
    db: tauri::State<'_, Arc<Database>>,
    request: UpdateTaskRequest,
) -> Result<Task, String> {
    let pool = db.get_pool();
    
    // 先获取任务的当前状态
    let old_status: Option<i32> = sqlx::query_scalar(
        "SELECT status FROM tasks WHERE id = ?"
    )
    .bind(request.id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    // 更新任务
    let service = TaskService::new(pool.clone());
    let result = service.update_task(request.clone()).await.map_err(|e| e.to_string())?;
    
    // 处理状态变更时的工作时长记录
    if let Some(old_status_val) = old_status {
        let new_status = if request.archived == Some(true) {
            4 // Archived
        } else if let Some(status) = request.status {
            i32::from(status)
        } else {
            old_status_val
        };
        
        if old_status_val != new_status {
            let work_service = crate::work_duration::WorkDurationService::new(pool.clone());
            if let Err(e) = work_service.handle_task_status_change(request.id, old_status_val, new_status).await {
                println!("⚠️ 工作时长记录处理失败：{}", e);
                // 不中断主流程，仅记录警告
            }
        }
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn delete_task(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
) -> Result<bool, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    service.delete_task(task_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
) -> Result<Task, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    service.get_task(task_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tasks_by_quadrant(
    db: tauri::State<'_, Arc<Database>>,
    quadrant: i32,
    include_archived: Option<bool>,
) -> Result<Vec<Task>, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    let quadrant = Quadrant::from(quadrant);
    let include_archived = include_archived.unwrap_or(false);
    
    service.get_tasks_by_quadrant(quadrant, include_archived).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recent_tasks(
    db: tauri::State<'_, Arc<Database>>,
    limit: i32,
) -> Result<Vec<Task>, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    service.get_recent_tasks(limit).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_tasks(
    db: tauri::State<'_, Arc<Database>>,
    query: String,
) -> Result<Vec<Task>, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    service.search_tasks(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task_status(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
    status: i32,
) -> Result<Task, String> {
    println!("📝 [update_task_status] 收到请求 - task_id: {}, status: {}", task_id, status);
    
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());

    let status = TaskStatus::from(status);
    println!("📝 [update_task_status] 转换后的状态: {:?}", status);
    
    match service.update_task_status(task_id, status).await {
        Ok(task) => {
            println!("✅ [update_task_status] 更新成功 - 任务：{} (ID: {})", task.title, task.id);
            Ok(task)
        }
        Err(e) => {
            println!("❌ [update_task_status] 更新失败：{}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_task_status_with_context(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
    old_status: i32,
    new_status: i32,
) -> Result<Task, String> {
    println!("📝 更新任务状态（带上下文）:");
    println!("   - 任务 ID: {}", task_id);
    println!("   - 原状态：{} ({})", old_status, status_code_to_string(old_status));
    println!("   - 新状态：{} ({})", new_status, status_code_to_string(new_status));

    let pool = db.get_pool();

    // 先处理工作时长记录
    {
        let work_service = crate::work_duration::WorkDurationService::new(pool.clone());
        if let Err(e) = work_service.handle_task_status_change(task_id, old_status, new_status).await {
            println!("⚠️ 工作时长记录处理失败：{}", e);
            // 不中断主流程，仅记录警告
        }
    }

    let service = TaskService::new(pool.clone());
    let status = TaskStatus::from(new_status);
    match service.update_task_status(task_id, status).await {
        Ok(task) => {
            println!("✅ 状态更新成功 - 任务：{} (ID: {})", task.title, task.id);
            println!("🎉 状态更新完成：{} → {}", status_code_to_string(old_status), status_code_to_string(new_status));
            Ok(task)
        }
        Err(e) => {
            println!("❌ 状态更新失败：{}", e);
            Err(e.to_string())
        }
    }
}

// 辅助函数：将状态码转换为字符串
fn status_code_to_string(code: i32) -> &'static str {
    match code {
        0 => "planning",
        1 => "in_progress",
        2 => "paused",
        3 => "completed",
        4 => "archived",
        _ => "unknown"
    }
}

#[tauri::command]
pub async fn get_tasks_by_status(
    db: tauri::State<'_, Arc<Database>>,
    status: i32,
    include_archived: Option<bool>,
) -> Result<Vec<Task>, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());

    let include_archived = include_archived.unwrap_or(false);
    service.get_tasks_by_status(TaskStatus::from(status), include_archived).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_tasks(
    db: tauri::State<'_, Arc<Database>>,
    include_archived: Option<bool>,
) -> Result<Vec<Task>, String> {
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());

    let include_archived = include_archived.unwrap_or(false);
    service.get_all_tasks(include_archived).await.map_err(|e| e.to_string())
}

/// 更新任务的四象限属性（重要性和紧急性）
/// 
/// # Arguments
/// * `task_id` - 任务ID
/// * `is_important` - 是否重要
/// * `is_urgent` - 是否紧急
/// 
/// # Returns
/// * 更新后的任务对象
#[tauri::command]
pub async fn update_task_quadrant(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
    is_important: bool,
    is_urgent: bool,
) -> Result<Task, String> {
    println!("📝 [update_task_quadrant] 收到请求 - task_id: {}, is_important: {}, is_urgent: {}", 
        task_id, is_important, is_urgent);
    
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    // 构建更新请求
    let request = UpdateTaskRequest {
        id: task_id,
        title: None,
        description: None,
        quadrant: None,
        status: None,
        progress: None,
        is_important: Some(is_important),
        is_urgent: Some(is_urgent),
        start_at: None,
        due_at: None,
        archived: None,
        add_tag_ids: None,
        remove_tag_ids: None,
    };
    
    match service.update_task(request).await {
        Ok(task) => {
            println!("✅ [update_task_quadrant] 更新成功 - 任务：{} (ID: {}), 象限: {:?}", 
                task.title, task.id, task.quadrant);
            Ok(task)
        }
        Err(e) => {
            println!("❌ [update_task_quadrant] 更新失败：{}", e);
            Err(e.to_string())
        }
    }
}

/// 归档或取消归档任务
/// 
/// # Arguments
/// * `task_id` - 任务ID
/// * `archived` - true表示归档，false表示取消归档
/// 
/// # Returns
/// * 更新后的任务对象
#[tauri::command]
pub async fn archive_task(
    db: tauri::State<'_, Arc<Database>>,
    task_id: i64,
    archived: bool,
) -> Result<Task, String> {
    println!("📝 [archive_task] 收到请求 - task_id: {}, archived: {}", task_id, archived);
    
    let pool = db.get_pool();
    let service = TaskService::new(pool.clone());
    
    // 构建更新请求
    let request = UpdateTaskRequest {
        id: task_id,
        title: None,
        description: None,
        quadrant: None,
        status: None,
        progress: None,
        is_important: None,
        is_urgent: None,
        start_at: None,
        due_at: None,
        archived: Some(archived),
        add_tag_ids: None,
        remove_tag_ids: None,
    };
    
    match service.update_task(request).await {
        Ok(task) => {
            let action = if archived { "归档" } else { "取消归档" };
            println!("✅ [archive_task] {}成功 - 任务：{} (ID: {})", action, task.title, task.id);
            Ok(task)
        }
        Err(e) => {
            println!("❌ [archive_task] 操作失败：{}", e);
            Err(e.to_string())
        }
    }
}
