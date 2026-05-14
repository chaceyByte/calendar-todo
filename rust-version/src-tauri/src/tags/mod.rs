use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::Manager;

use crate::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub task_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

pub struct TagService {
    pool: SqlitePool,
}

impl TagService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create_tag(&self, req: CreateTagRequest) -> Result<Tag, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();
        
        let tag_id = sqlx::query(
            "INSERT INTO tags (name, color, user_id, created_at, updated_at)
             VALUES (?, ?, 1, ?, ?)"
        )
        .bind(&req.name)
        .bind(&req.color)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_tag(tag_id).await
    }
    
    pub async fn update_tag(&self, req: UpdateTagRequest) -> Result<Tag, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();

        if let Some(name) = &req.name {
            sqlx::query("UPDATE tags SET name = ?, updated_at = ? WHERE id = ?")
                .bind(name)
                .bind(now)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(color) = &req.color {
            sqlx::query("UPDATE tags SET color = ?, updated_at = ? WHERE id = ?")
                .bind(color)
                .bind(now)
                .bind(req.id)
                .execute(&self.pool)
                .await?;
        }

        self.get_tag(req.id).await
    }
    
    pub async fn delete_tag(&self, tag_id: i64) -> Result<bool, sqlx::Error> {
        let usage_count = self.get_tag_usage_count(tag_id).await?;

        if usage_count > 0 {
            return Ok(false);
        }

        let result = sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(tag_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
    
    pub async fn get_tags(&self) -> Result<Vec<Tag>, sqlx::Error> {
        sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE user_id = 1 ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_tags_with_count(&self) -> Result<Vec<TagWithCount>, sqlx::Error> {
        let tags: Vec<Tag> = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for tag in tags {
            let task_count = self.get_tag_usage_count(tag.id).await?;
            result.push(TagWithCount {
                id: tag.id,
                name: tag.name,
                color: tag.color,
                task_count,
            });
        }

        Ok(result)
    }

    pub async fn get_tag(&self, tag_id: i64) -> Result<Tag, sqlx::Error> {
        sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE id = ?"
        )
        .bind(tag_id)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn add_tag_to_task(&self, task_id: i64, tag_id: i64) -> Result<bool, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();
        
        let result = sqlx::query(
            "INSERT OR IGNORE INTO task_tags (task_id, tag_id, created_at) VALUES (?, ?, ?)"
        )
        .bind(task_id)
        .bind(tag_id)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    pub async fn remove_tag_from_task(&self, task_id: i64, tag_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?")
            .bind(task_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_tag_usage_count(&self, tag_id: i64) -> Result<i64, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM task_tags WHERE tag_id = ?"
        )
        .bind(tag_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    pub async fn get_task_tags(&self, task_id: i64) -> Result<Vec<Tag>, sqlx::Error> {
        sqlx::query_as::<_, Tag>(
            "SELECT t.* FROM tags t
             INNER JOIN task_tags tt ON t.id = tt.tag_id
             WHERE tt.task_id = ?
             ORDER BY t.name"
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
    }
}

#[tauri::command]
pub async fn create_tag(
    app_handle: tauri::AppHandle,
    request: CreateTagRequest,
) -> Result<Tag, String> {
    println!("[create_tag] 收到请求: {:?}", request);
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    match service.create_tag(request).await {
        Ok(tag) => {
            println!("[create_tag] 创建成功: {:?}", tag);
            Ok(tag)
        }
        Err(e) => {
            println!("[create_tag] 创建失败: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_tag(
    app_handle: tauri::AppHandle,
    request: UpdateTagRequest,
) -> Result<Tag, String> {
    println!("[update_tag] 收到请求: {:?}", request);
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    match service.update_tag(request).await {
        Ok(tag) => {
            println!("[update_tag] 更新成功: {:?}", tag);
            Ok(tag)
        }
        Err(e) => {
            println!("[update_tag] 更新失败: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn delete_tag(
    app_handle: tauri::AppHandle,
    tag_id: i64,
) -> Result<bool, String> {
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    service.delete_tag(tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tags(
    app_handle: tauri::AppHandle,
) -> Result<Vec<TagWithCount>, String> {
    println!("[get_tags] 开始获取标签");
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    match service.get_tags_with_count().await {
        Ok(tags) => {
            println!("[get_tags] 获取成功，共 {} 个标签", tags.len());
            Ok(tags)
        }
        Err(e) => {
            println!("[get_tags] 获取失败: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn add_tag_to_task(
    app_handle: tauri::AppHandle,
    task_id: i64,
    tag_id: i64,
) -> Result<bool, String> {
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    service.add_tag_to_task(task_id, tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_tag_from_task(
    app_handle: tauri::AppHandle,
    task_id: i64,
    tag_id: i64,
) -> Result<bool, String> {
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    service.remove_tag_from_task(task_id, tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_tags(
    app_handle: tauri::AppHandle,
    task_id: i64,
) -> Result<Vec<Tag>, String> {
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    service.get_task_tags(task_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tag_usage_count(
    app_handle: tauri::AppHandle,
    tag_id: i64,
) -> Result<i64, String> {
    let db = app_handle.state::<Arc<Database>>();
    let pool = db.get_pool();
    let service = TagService::new(pool.clone());

    service.get_tag_usage_count(tag_id).await.map_err(|e| e.to_string())
}