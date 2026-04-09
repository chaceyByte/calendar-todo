use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: i64,
    pub r#type: String, // "daily", "weekly"
    pub title: String,
    pub content: Option<String>,
    pub report_date: chrono::NaiveDate,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReportRequest {
    pub r#type: String,
    pub title: String,
    pub content: String,
    pub report_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportStats {
    pub total_reports: i32,
    pub daily_reports: i32,
    pub weekly_reports: i32,
    pub last_report_date: Option<String>,
}

pub struct ReportService {
    pool: SqlitePool,
}

impl ReportService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn save_report(&self, req: CreateReportRequest) -> Result<Report, sqlx::Error> {
        let now = Utc::now().naive_utc();
        let report_date = chrono::NaiveDate::parse_from_str(&req.report_date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            
        let report_id = sqlx::query(
            "INSERT INTO reports (type, title, content, report_date, user_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, 1, ?, ?)"
        )
        .bind(&req.r#type)
        .bind(&req.title)
        .bind(&req.content)
        .bind(report_date)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_report(report_id).await
    }
    
    pub async fn get_reports(&self, r#type: Option<String>, limit: Option<i32>) -> Result<Vec<Report>, sqlx::Error> {
        let mut query = "SELECT r.* FROM reports r WHERE 1=1".to_string();
        let mut params = Vec::new();
        
        if let Some(t) = &r#type {
            query.push_str(" AND r.type = ?");
            params.push(t as &dyn sqlx::encode::Encode<'_>);
        }
        
        query.push_str(" ORDER BY r.report_date DESC");
        
        if let Some(l) = limit {
            query.push_str(" LIMIT ?");
            params.push(&l as &dyn sqlx::encode::Encode<'_>);
        }
        
        let mut sqlx_query = sqlx::query_as::<_, Report>(&query);
        for param in params {
            sqlx_query = sqlx_query.bind(param);
        }
        
        sqlx_query.fetch_all(&self.pool).await
    }
    
    pub async fn get_report_stats(&self) -> Result<ReportStats, sqlx::Error> {
        let total_reports = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM reports")
            .fetch_one(&self.pool)
            .await?;
            
        let daily_reports = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM reports WHERE type = 'daily'")
            .fetch_one(&self.pool)
            .await?;
            
        let weekly_reports = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM reports WHERE type = 'weekly'")
            .fetch_one(&self.pool)
            .await?;
            
        let last_report_date = sqlx::query_scalar::<_, Option<String>>(
            "SELECT report_date FROM reports ORDER BY report_date DESC LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(ReportStats {
            total_reports,
            daily_reports,
            weekly_reports,
            last_report_date,
        })
    }
    
    async fn get_report(&self, report_id: i64) -> Result<Report, sqlx::Error> {
        sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE id = ?"
        )
        .bind(report_id)
        .fetch_one(&self.pool)
        .await
    }
}

#[tauri::command]
pub async fn save_report(
    app_handle: tauri::AppHandle,
    request: CreateReportRequest,
) -> Result<Report, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ReportService::new(pool.clone());
    
    service.save_report(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reports(
    app_handle: tauri::AppHandle,
    r#type: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<Report>, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ReportService::new(pool.clone());
    
    service.get_reports(r#type, limit).await.map_err(|e| e.to_string())
}