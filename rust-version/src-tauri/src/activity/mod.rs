use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityRecord {
    pub id: i64,
    pub task_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub activity_type: ActivityType,
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub user_id: i64,
    pub initial_status: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActivityType {
    Created,
    Started,
    Paused,
    Resumed,
    Completed,
    Work,
    Meeting,
    Study,
    Other,
}

impl From<String> for ActivityType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "created" => ActivityType::Created,
            "started" => ActivityType::Started,
            "paused" => ActivityType::Paused,
            "resumed" => ActivityType::Resumed,
            "completed" => ActivityType::Completed,
            "work" => ActivityType::Work,
            "meeting" => ActivityType::Meeting,
            "study" => ActivityType::Study,
            "other" => ActivityType::Other,
            _ => ActivityType::Other,
        }
    }
}

impl From<ActivityType> for String {
    fn from(value: ActivityType) -> Self {
        match value {
            ActivityType::Created => "created".to_string(),
            ActivityType::Started => "started".to_string(),
            ActivityType::Paused => "paused".to_string(),
            ActivityType::Resumed => "resumed".to_string(),
            ActivityType::Completed => "completed".to_string(),
            ActivityType::Work => "work".to_string(),
            ActivityType::Meeting => "meeting".to_string(),
            ActivityType::Study => "study".to_string(),
            ActivityType::Other => "other".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateActivityRequest {
    pub task_id: i64,
    pub activity_type: String,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReport {
    pub date: String,
    pub total_activities: i32,
    pub total_duration_minutes: i32,
    pub activities: Vec<ActivityRecord>,
    pub completed_tasks: i32,
    pub active_tasks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyReport {
    pub week_start: String,
    pub week_end: String,
    pub daily_reports: Vec<DailyReport>,
    pub total_activities: i32,
    pub total_duration_hours: f32,
    pub average_daily_minutes: f32,
}

pub struct ActivityService {
    pool: SqlitePool,
}

impl ActivityService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create_activity_record(&self, req: CreateActivityRequest) -> Result<ActivityRecord, sqlx::Error> {
        let now = Utc::now();
        let start_time = req.start_time
            .map(|s| chrono::DateTime::parse_from_rfc3339(&s))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
            
        let end_time = req.end_time
            .map(|s| chrono::DateTime::parse_from_rfc3339(&s))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .map(|dt| dt.with_timezone(&Utc));
            
        let activity_type = ActivityType::from(req.activity_type.clone());
        
        let activity_id = sqlx::query(
            "INSERT INTO activity_records (task_id, start_time, end_time, activity_type, description, duration_minutes, user_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 1, ?)"
        )
        .bind(req.task_id)
        .bind(start_time.naive_utc())
        .bind(end_time.map(|dt| dt.naive_utc()))
        .bind(String::from(activity_type))
        .bind(req.description)
        .bind(req.duration_minutes)
        .bind(now.naive_utc())
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_activity(activity_id).await
    }
    
    pub async fn start_activity(&self, task_id: i64, description: Option<String>) -> Result<ActivityRecord, sqlx::Error> {
        let req = CreateActivityRequest {
            task_id,
            activity_type: "started".to_string(),
            description,
            start_time: Some(Utc::now().to_rfc3339()),
            end_time: None,
            duration_minutes: None,
        };
        
        self.create_activity_record(req).await
    }
    
    pub async fn pause_activity(&self, task_id: i64, description: Option<String>) -> Result<ActivityRecord, sqlx::Error> {
        // 查找当前正在进行的活动并结束它
        let ongoing_activity = self.find_ongoing_activity(task_id).await;
        if let Ok(mut activity) = ongoing_activity {
            let now = Utc::now();
            activity.end_time = Some(now);
            
            // 计算持续时间
            let duration_seconds = now.signed_duration_since(activity.start_time).num_seconds();
            let duration_minutes = (duration_seconds / 60) as i32;
            
            sqlx::query(
                "UPDATE activity_records SET end_time = ?, duration_minutes = ? WHERE id = ?"
            )
            .bind(now.naive_utc())
            .bind(duration_minutes)
            .bind(activity.id)
            .execute(&self.pool)
            .await?;
        }
        
        // 创建暂停记录
        let req = CreateActivityRequest {
            task_id,
            activity_type: "paused".to_string(),
            description,
            start_time: Some(Utc::now().to_rfc3339()),
            end_time: None,
            duration_minutes: None,
        };
        
        self.create_activity_record(req).await
    }
    
    pub async fn resume_activity(&self, task_id: i64, description: Option<String>) -> Result<ActivityRecord, sqlx::Error> {
        let req = CreateActivityRequest {
            task_id,
            activity_type: "resumed".to_string(),
            description,
            start_time: Some(Utc::now().to_rfc3339()),
            end_time: None,
            duration_minutes: None,
        };
        
        self.create_activity_record(req).await
    }
    
    pub async fn complete_activity(&self, task_id: i64, description: Option<String>) -> Result<ActivityRecord, sqlx::Error> {
        // 结束当前正在进行的活动
        let ongoing_activity = self.find_ongoing_activity(task_id).await;
        if let Ok(mut activity) = ongoing_activity {
            let now = Utc::now();
            activity.end_time = Some(now);
            
            let duration_seconds = now.signed_duration_since(activity.start_time).num_seconds();
            let duration_minutes = (duration_seconds / 60) as i32;
            
            sqlx::query(
                "UPDATE activity_records SET end_time = ?, duration_minutes = ? WHERE id = ?"
            )
            .bind(now.naive_utc())
            .bind(duration_minutes)
            .bind(activity.id)
            .execute(&self.pool)
            .await?;
        }
        
        // 创建完成记录
        let req = CreateActivityRequest {
            task_id,
            activity_type: "completed".to_string(),
            description,
            start_time: Some(Utc::now().to_rfc3339()),
            end_time: None,
            duration_minutes: None,
        };
        
        self.create_activity_record(req).await
    }
    
    pub async fn get_task_activities(&self, task_id: i64) -> Result<Vec<ActivityRecord>, sqlx::Error> {
        sqlx::query_as::<_, ActivityRecord>(
            "SELECT ar.* FROM activity_records ar
             WHERE ar.task_id = ?
             ORDER BY ar.start_time DESC"
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
    }
    
    pub async fn get_daily_activities(&self, date: String) -> Result<Vec<ActivityRecord>, sqlx::Error> {
        let date_naive = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let next_day = date_naive.succ_opt().unwrap_or(date_naive);
        
        sqlx::query_as::<_, ActivityRecord>(
            "SELECT ar.* FROM activity_records ar
             WHERE DATE(ar.start_time) = DATE(?)
             ORDER BY ar.start_time ASC"
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await
    }
    
    pub async fn generate_daily_report(&self, date: String) -> Result<DailyReport, sqlx::Error> {
        let activities = self.get_daily_activities(date.clone()).await?;
        
        let total_duration_minutes = activities.iter()
            .filter_map(|a| a.duration_minutes)
            .sum();
            
        // 获取当天完成的任务数
        let completed_tasks = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(DISTINCT task_id) FROM activity_records 
             WHERE DATE(start_time) = DATE(?) 
               AND activity_type = 'completed'"
        )
        .bind(&date)
        .fetch_one(&self.pool)
        .await?;
        
        // 获取当天活跃的任务数
        let active_tasks = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(DISTINCT task_id) FROM activity_records 
             WHERE DATE(start_time) = DATE(?)"
        )
        .bind(&date)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(DailyReport {
            date,
            total_activities: activities.len() as i32,
            total_duration_minutes,
            activities,
            completed_tasks,
            active_tasks,
        })
    }
    
    pub async fn generate_weekly_report(&self, week_start: String) -> Result<WeeklyReport, sqlx::Error> {
        let week_start_date = chrono::NaiveDate::parse_from_str(&week_start, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            
        let mut week_end_date = week_start_date;
        for _ in 0..6 {
            week_end_date = week_end_date.succ_opt().unwrap_or(week_end_date);
        }
        
        let week_end = week_end_date.format("%Y-%m-%d").to_string();
        
        let mut daily_reports = Vec::new();
        let mut current_date = week_start_date;
        let mut total_activities = 0;
        let mut total_duration_minutes = 0;
        
        while current_date <= week_end_date {
            let date_str = current_date.format("%Y-%m-%d").to_string();
            let daily_report = self.generate_daily_report(date_str.clone()).await?;
            
            total_activities += daily_report.total_activities;
            total_duration_minutes += daily_report.total_duration_minutes;
            
            daily_reports.push(daily_report);
            current_date = current_date.succ_opt().unwrap_or(current_date);
        }
        
        let average_daily_minutes = if daily_reports.is_empty() {
            0.0
        } else {
            total_duration_minutes as f32 / daily_reports.len() as f32
        };
        
        Ok(WeeklyReport {
            week_start,
            week_end,
            daily_reports,
            total_activities,
            total_duration_hours: total_duration_minutes as f32 / 60.0,
            average_daily_minutes,
        })
    }
    
    async fn find_ongoing_activity(&self, task_id: i64) -> Result<ActivityRecord, sqlx::Error> {
        sqlx::query_as::<_, ActivityRecord>(
            "SELECT ar.* FROM activity_records ar
             WHERE ar.task_id = ? 
               AND ar.end_time IS NULL 
               AND ar.activity_type IN ('started', 'resumed')
             ORDER BY ar.start_time DESC
             LIMIT 1"
        )
        .bind(task_id)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn get_activity(&self, activity_id: i64) -> Result<ActivityRecord, sqlx::Error> {
        sqlx::query_as::<_, ActivityRecord>(
            "SELECT * FROM activity_records WHERE id = ?"
        )
        .bind(activity_id)
        .fetch_one(&self.pool)
        .await
    }
}

#[tauri::command]
pub async fn create_activity_record(
    app_handle: tauri::AppHandle,
    request: CreateActivityRequest,
) -> Result<ActivityRecord, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.create_activity_record(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_activity(
    app_handle: tauri::AppHandle,
    task_id: i64,
    description: Option<String>,
) -> Result<ActivityRecord, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.start_activity(task_id, description).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_activity(
    app_handle: tauri::AppHandle,
    task_id: i64,
    description: Option<String>,
) -> Result<ActivityRecord, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.pause_activity(task_id, description).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn resume_activity(
    app_handle: tauri::AppHandle,
    task_id: i64,
    description: Option<String>,
) -> Result<ActivityRecord, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.resume_activity(task_id, description).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn complete_activity(
    app_handle: tauri::AppHandle,
    task_id: i64,
    description: Option<String>,
) -> Result<ActivityRecord, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.complete_activity(task_id, description).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_activities(
    app_handle: tauri::AppHandle,
    task_id: i64,
) -> Result<Vec<ActivityRecord>, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.get_task_activities(task_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_daily_activities(
    app_handle: tauri::AppHandle,
    date: String,
) -> Result<Vec<ActivityRecord>, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.get_daily_activities(date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_daily_report(
    app_handle: tauri::AppHandle,
    date: String,
) -> Result<DailyReport, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.generate_daily_report(date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_weekly_report(
    app_handle: tauri::AppHandle,
    week_start: String,
) -> Result<WeeklyReport, String> {
    let db = app_handle.state::<Database>();
    let pool = db.get_pool();
    let service = ActivityService::new(pool.clone());
    
    service.generate_weekly_report(week_start).await.map_err(|e| e.to_string())
}