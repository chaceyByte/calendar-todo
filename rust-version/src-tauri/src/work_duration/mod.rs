use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

use crate::database::Database;

/// 工作记录类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkRecordType {
    Started,   // 开始工作
    Paused,    // 暂停
    Completed, // 完成
    Archived,  // 归档
}

impl WorkRecordType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkRecordType::Started => "started",
            WorkRecordType::Paused => "paused",
            WorkRecordType::Completed => "completed",
            WorkRecordType::Archived => "archived",
        }
    }
}

/// 任务工作记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskWorkRecord {
    pub id: i64,
    pub task_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration_minutes: i64,
    pub work_days_minutes: i64,
    pub record_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 创建/更新工作记录请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkRecordRequest {
    pub task_id: i64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub record_type: String,
}

/// 工作时长统计
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDurationStats {
    pub total_archived: i64,
    pub average_lifetime_days: f64,
}

/// 归档任务查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveQueryParams {
    pub page: i32,
    pub page_size: i32,
    pub keyword: Option<String>,
}

/// 归档任务列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveTaskListResponse {
    pub tasks: Vec<ArchiveTaskItem>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub stats: WorkDurationStats,
}

/// 归档任务项
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ArchiveTaskItem {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub quadrant: i32,
    pub created_at: NaiveDateTime,
    pub archived_at: Option<NaiveDateTime>,
    pub total_work_duration_minutes: i64,
    pub tags: Option<String>, // JSON 字符串
}

impl ArchiveTaskItem {
    /// 将分钟转换为工作天数（8小时/天）
    pub fn total_work_days(&self) -> f64 {
        self.total_work_duration_minutes as f64 / 480.0 // 480分钟 = 8小时
    }
}

/// 工作时长计算服务
pub struct WorkDurationService {
    pool: SqlitePool,
}

impl WorkDurationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 计算两个日期之间的工作日天数（考虑节假日和补班）
    /// 
    /// 规则：
    /// - 每天8小时工作制
    /// - 排除周末（周六、周日）
    /// - 排除节假日（休假）
    /// - 加上补班日（周末调整为工作日）
    pub async fn calculate_work_days(
        &self,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<f64, sqlx::Error> {
        let start_date = start_time.date();
        let end_date = end_time.date();
        
        // 获取日期范围内的所有特殊日期配置
        let holiday_configs = self.get_holiday_configs_in_range(
            &start_date.format("%Y-%m-%d").to_string(),
            &end_date.format("%Y-%m-%d").to_string(),
        ).await?;

        // 构建日期类型映射
        let mut date_type_map = std::collections::HashMap::new();
        for config in &holiday_configs {
            date_type_map.insert(config.date, config.date_type.as_str());
        }

        // 计算工作日天数
        let mut work_days = 0.0;
        let mut current_date = start_date;

        while current_date <= end_date {
            let day_of_week = current_date.weekday().num_days_from_monday() as i32;
            let is_weekend = day_of_week >= 5; // 周六(5)或周日(6)
            
            // 判断当天是否是工作日
            let is_workday = match date_type_map.get(&current_date) {
                Some(date_type) => match *date_type {
                    "holiday" => false, // 休假
                    "makeup" => true,   // 补班
                    _ => !is_weekend,   // 其他按默认规则
                },
                None => !is_weekend, // 无配置按默认规则
            };

            if is_workday {
                // 计算当天的工作小时数
                let work_hours = if current_date == start_date && current_date == end_date {
                    // 开始和结束是同一天
                    let start_hour = start_time.hour() as f64 + start_time.minute() as f64 / 60.0;
                    let end_hour = end_time.hour() as f64 + end_time.minute() as f64 / 60.0;
                    let hours = end_hour - start_hour;
                    hours.min(8.0).max(0.0) // 最多8小时，最少0小时
                } else if current_date == start_date {
                    // 开始当天
                    let start_hour = start_time.hour() as f64 + start_time.minute() as f64 / 60.0;
                    let hours = 18.0 - start_hour.max(9.0); // 假设工作时间 9:00-18:00
                    hours.min(8.0).max(0.0)
                } else if current_date == end_date {
                    // 结束当天
                    let end_hour = end_time.hour() as f64 + end_time.minute() as f64 / 60.0;
                    let hours = end_hour.min(18.0) - 9.0; // 假设工作时间 9:00-18:00
                    hours.min(8.0).max(0.0)
                } else {
                    // 中间完整的一天
                    8.0
                };
                
                work_days += work_hours / 8.0;
            }

            current_date = match current_date.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(work_days)
    }

    /// 获取日期范围内的节假日配置
    async fn get_holiday_configs_in_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HolidayConfigRow>, sqlx::Error> {
        sqlx::query_as::<_, HolidayConfigRow>(
            "SELECT date, type as date_type FROM holiday_configs WHERE date >= ? AND date <= ?"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
    }

    /// 创建或更新工作记录
    pub async fn create_work_record(
        &self,
        req: CreateWorkRecordRequest,
    ) -> Result<TaskWorkRecord, sqlx::Error> {
        let start_time = NaiveDateTime::parse_from_str(&req.start_time, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let end_time = match req.end_time {
            Some(et) => Some(
                NaiveDateTime::parse_from_str(&et, "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            ),
            None => None,
        };

        // 计算工作时长
        let (duration_minutes, work_days_minutes) = if let Some(et) = end_time {
            let duration = et.signed_duration_since(start_time);
            let minutes = duration.num_minutes();
            // 计算有效工作分钟数
            let effective_minutes = self.calculate_effective_work_minutes(start_time, et).await?;
            (minutes, effective_minutes)
        } else {
            (0, 0)
        };

        let now = Utc::now().naive_utc();

        // 插入工作记录
        let id: i64 = sqlx::query(
            "INSERT INTO task_work_records (task_id, start_time, end_time, duration_minutes, work_days_minutes, record_type, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id"
        )
        .bind(req.task_id)
        .bind(start_time)
        .bind(end_time)
        .bind(duration_minutes)
        .bind(work_days_minutes)
        .bind(&req.record_type)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        // 更新任务的总工作时长缓存
        self.update_task_work_duration_cache(req.task_id).await?;

        sqlx::query_as::<_, TaskWorkRecord>(
            "SELECT * FROM task_work_records WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    /// 关闭未完成的工作记录（用于暂停或完成任务时）
    pub async fn close_active_work_record(
        &self,
        task_id: i64,
        end_time: NaiveDateTime,
        record_type: &str,
    ) -> Result<Option<TaskWorkRecord>, sqlx::Error> {
        // 查找未完成的记录
        let active_record: Option<TaskWorkRecord> = sqlx::query_as(
            "SELECT * FROM task_work_records WHERE task_id = ? AND end_time IS NULL ORDER BY start_time DESC LIMIT 1"
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(record) = active_record {
            // 计算有效工作时长（考虑工作时间 9:00-18:00，排除周末和节假日）
            let (duration_minutes, work_days_minutes) = self.calculate_effective_work_time(record.start_time, end_time).await?;

            // 更新记录
            sqlx::query(
                "UPDATE task_work_records SET end_time = ?, duration_minutes = ?, work_days_minutes = ?, record_type = ?, updated_at = ? WHERE id = ?"
            )
            .bind(end_time)
            .bind(duration_minutes)
            .bind(work_days_minutes)
            .bind(record_type)
            .bind(Utc::now().naive_utc())
            .bind(record.id)
            .execute(&self.pool)
            .await?;

            // 更新任务缓存
            self.update_task_work_duration_cache(task_id).await?;

            // 返回更新后的记录
            let updated: TaskWorkRecord = sqlx::query_as(
                "SELECT * FROM task_work_records WHERE id = ?"
            )
            .bind(record.id)
            .fetch_one(&self.pool)
            .await?;

            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    /// 计算有效工作时间和天数
    /// 
    /// 规则：
    /// - 工作时间：9:00-18:00（9小时，但按8小时/天计算）
    /// - 排除周末（周六、周日）
    /// - 排除节假日（休假）
    /// - 加上补班日（周末调整为工作日）
    /// 
    /// 返回：(实际经过分钟数, 有效工作分钟数)
    async fn calculate_effective_work_time(
        &self,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<(i64, i64), sqlx::Error> {
        let duration_minutes = end_time.signed_duration_since(start_time).num_minutes();
        let effective_minutes = self.calculate_effective_work_minutes(start_time, end_time).await?;
        Ok((duration_minutes, effective_minutes))
    }

    /// 计算有效工作分钟数
    /// 
    /// 规则：
    /// - 工作时间：9:00-18:00
    /// - 排除周末（周六、周日）
    /// - 排除节假日（休假）
    /// - 加上补班日（周末调整为工作日）
    async fn calculate_effective_work_minutes(
        &self,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<i64, sqlx::Error> {
        let start_date = start_time.date();
        let end_date = end_time.date();
        
        // 获取日期范围内的所有特殊日期配置
        let holiday_configs = self.get_holiday_configs_in_range(
            &start_date.format("%Y-%m-%d").to_string(),
            &end_date.format("%Y-%m-%d").to_string(),
        ).await?;

        // 构建日期类型映射
        let mut date_type_map = std::collections::HashMap::new();
        for config in &holiday_configs {
            date_type_map.insert(config.date, config.date_type.as_str());
        }

        // 计算有效工作分钟数
        let mut total_minutes: i64 = 0;
        let mut current_date = start_date;

        while current_date <= end_date {
            let day_of_week = current_date.weekday().num_days_from_monday() as i32;
            let is_weekend = day_of_week >= 5; // 周六(5)或周日(6)
            
            // 判断当天是否是工作日
            let is_workday = match date_type_map.get(&current_date) {
                Some(date_type) => match *date_type {
                    "holiday" => false, // 休假
                    "makeup" => true,   // 补班
                    _ => !is_weekend,   // 其他按默认规则
                },
                None => !is_weekend, // 无配置按默认规则
            };

            if is_workday {
                // 定义工作时间范围（9:00-18:00）
                let work_start_minute: i64 = 9 * 60; // 540分钟
                let work_end_minute: i64 = 18 * 60;  // 1080分钟
                let daily_work_minutes: i64 = 480;   // 8小时 = 480分钟

                // 计算当天的工作分钟数
                let day_minutes: i64 = if current_date == start_date && current_date == end_date {
                    // 开始和结束是同一天
                    let start_minute = ((start_time.hour() as i64 * 60 + start_time.minute() as i64)
                        .max(work_start_minute))
                        .min(work_end_minute);
                    let end_minute = ((end_time.hour() as i64 * 60 + end_time.minute() as i64)
                        .max(work_start_minute))
                        .min(work_end_minute);
                    (end_minute - start_minute).max(0)
                } else if current_date == start_date {
                    // 开始当天：从 start_time 到 18:00
                    let start_minute = ((start_time.hour() as i64 * 60 + start_time.minute() as i64)
                        .max(work_start_minute))
                        .min(work_end_minute);
                    work_end_minute - start_minute
                } else if current_date == end_date {
                    // 结束当天：从 9:00 到 end_time
                    let end_minute = ((end_time.hour() as i64 * 60 + end_time.minute() as i64)
                        .max(work_start_minute))
                        .min(work_end_minute);
                    end_minute - work_start_minute
                } else {
                    // 中间完整的一天
                    daily_work_minutes
                };
                
                total_minutes += day_minutes.max(0);
            }

            current_date = match current_date.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(total_minutes)
    }

    /// 更新任务的工作时长缓存
    async fn update_task_work_duration_cache(&self, task_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE tasks SET 
                total_work_duration_minutes = COALESCE((SELECT SUM(duration_minutes) FROM task_work_records WHERE task_id = ?), 0)
             WHERE id = ?"
        )
        .bind(task_id)
        .bind(task_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 处理任务状态变更
    pub async fn handle_task_status_change(
        &self,
        task_id: i64,
        old_status: i32,
        new_status: i32,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();

        // 状态定义：0=规划中, 1=进行中, 2=已暂停, 3=已完成, 4=已归档
        match (old_status, new_status) {
            // 规划中 -> 进行中：开始新的工作记录
            (0, 1) => {
                // 更新任务实际开始时间
                sqlx::query("UPDATE tasks SET actual_start_at = ? WHERE id = ?")
                    .bind(now)
                    .bind(task_id)
                    .execute(&self.pool)
                    .await?;

                // 创建工作记录
                self.create_work_record(CreateWorkRecordRequest {
                    task_id,
                    start_time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                    end_time: None,
                    record_type: "started".to_string(),
                }).await?;
            }
            // 进行中 -> 已暂停：关闭当前工作记录
            (1, 2) => {
                self.close_active_work_record(task_id, now, "paused").await?;
            }
            // 已暂停 -> 进行中：开始新的工作记录
            (2, 1) => {
                self.create_work_record(CreateWorkRecordRequest {
                    task_id,
                    start_time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                    end_time: None,
                    record_type: "started".to_string(),
                }).await?;
            }
            // 进行中 -> 已完成：关闭当前工作记录
            (1, 3) => {
                self.close_active_work_record(task_id, now, "completed").await?;
            }
            // 进行中 -> 已归档：关闭当前工作记录
            (1, 4) => {
                self.close_active_work_record(task_id, now, "archived").await?;
            }
            // 已暂停 -> 已归档：关闭当前工作记录（暂停时未关闭的记录）
            (2, 4) => {
                self.close_active_work_record(task_id, now, "archived").await?;
            }
            // 已完成 -> 已归档：无需处理（已完成时已记录）
            (3, 4) => {}
            // 其他状态变更不处理
            _ => {}
        }

        Ok(())
    }

    /// 获取已归档任务列表
    pub async fn get_archived_tasks(
        &self,
        params: ArchiveQueryParams,
    ) -> Result<ArchiveTaskListResponse, sqlx::Error> {
        let offset = (params.page - 1) * params.page_size;
        
        // 构建查询条件
        let where_clause = if params.keyword.as_ref().map_or(false, |k| !k.is_empty()) {
            "WHERE t.status = 4 AND (t.title LIKE ? OR t.description LIKE ?)"
        } else {
            "WHERE t.status = 4"
        };

        // 查询总数
        let total_sql = format!(
            "SELECT COUNT(*) FROM tasks t {}",
            where_clause
        );
        
        let total: i64 = if params.keyword.as_ref().map_or(false, |k| !k.is_empty()) {
            let keyword = format!("%{}%", params.keyword.as_ref().unwrap());
            sqlx::query_scalar(&total_sql)
                .bind(&keyword)
                .bind(&keyword)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_scalar(&total_sql)
                .fetch_one(&self.pool)
                .await?
        };

        // 查询任务列表（包含标签）
        let tasks_sql = format!(
            "SELECT 
                t.id,
                t.title,
                t.description,
                t.quadrant,
                t.created_at,
                t.archived_at,
                t.total_work_duration_minutes,
                (SELECT GROUP_CONCAT(json_object('id', tg.id, 'name', tg.name, 'color', tg.color))
                 FROM tags tg
                 INNER JOIN task_tags tt ON tg.id = tt.tag_id
                 WHERE tt.task_id = t.id) as tags
             FROM tasks t
             {}
             ORDER BY t.archived_at DESC
             LIMIT ? OFFSET ?",
            where_clause
        );

        let tasks: Vec<ArchiveTaskItem> = if params.keyword.as_ref().map_or(false, |k| !k.is_empty()) {
            let keyword = format!("%{}%", params.keyword.as_ref().unwrap());
            sqlx::query_as(&tasks_sql)
                .bind(&keyword)
                .bind(&keyword)
                .bind(params.page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as(&tasks_sql)
                .bind(params.page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        };

        // 计算统计数据
        let stats = self.calculate_archive_stats().await?;

        Ok(ArchiveTaskListResponse {
            tasks,
            total,
            page: params.page,
            page_size: params.page_size,
            stats,
        })
    }

    /// 计算归档统计
    async fn calculate_archive_stats(&self) -> Result<WorkDurationStats, sqlx::Error> {
        // 总归档量
        let total_archived: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE status = 4"
        )
        .fetch_one(&self.pool)
        .await?;

        // 平均寿命（天数）- 从分钟转换为天
        let average_minutes: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(total_work_duration_minutes) FROM tasks WHERE status = 4 AND total_work_duration_minutes > 0"
        )
        .fetch_one(&self.pool)
        .await?;

        // 将分钟转换为天（8小时/天 = 480分钟/天）
        let average_lifetime_days = average_minutes.unwrap_or(0.0) / 480.0;

        Ok(WorkDurationStats {
            total_archived,
            average_lifetime_days,
        })
    }

    /// 获取任务的工作记录列表
    pub async fn get_task_work_records(
        &self,
        task_id: i64,
    ) -> Result<Vec<TaskWorkRecord>, sqlx::Error> {
        sqlx::query_as::<_, TaskWorkRecord>(
            "SELECT * FROM task_work_records WHERE task_id = ? ORDER BY start_time DESC"
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
    }
}

/// 节假日配置行（内部使用）
#[derive(Debug, FromRow)]
struct HolidayConfigRow {
    pub date: NaiveDate,
    pub date_type: String,
}

// ==================== Tauri Commands ====================

#[tauri::command]
pub async fn create_work_record(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: CreateWorkRecordRequest,
) -> Result<TaskWorkRecord, String> {
    let pool = db.get_pool();
    let service = WorkDurationService::new(pool.clone());
    
    service.create_work_record(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_active_work_record(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    task_id: i64,
    record_type: String,
) -> Result<Option<TaskWorkRecord>, String> {
    let pool = db.get_pool();
    let service = WorkDurationService::new(pool.clone());
    let now = Utc::now().naive_utc();
    
    service.close_active_work_record(task_id, now, &record_type).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_work_records(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    task_id: i64,
) -> Result<Vec<TaskWorkRecord>, String> {
    let pool = db.get_pool();
    let service = WorkDurationService::new(pool.clone());
    
    service.get_task_work_records(task_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_archived_tasks(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    page: i32,
    page_size: i32,
    keyword: Option<String>,
) -> Result<ArchiveTaskListResponse, String> {
    let pool = db.get_pool();
    let service = WorkDurationService::new(pool.clone());
    
    let params = ArchiveQueryParams {
        page,
        page_size,
        keyword,
    };
    
    service.get_archived_tasks(params).await.map_err(|e| e.to_string())
}

/// 获取指定日期的归档任务（用于日历日视图显示）
#[tauri::command]
pub async fn get_archived_tasks_by_date(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<Vec<ArchiveTaskItem>, String> {
    let pool = db.get_pool();
    
    // 查询在指定日期归档的任务
    // archived_at 字段格式为 "2024-01-15 10:30:00"，需要匹配日期部分
    let tasks = sqlx::query_as::<_, ArchiveTaskItem>(
        "SELECT 
            t.id,
            t.title,
            t.description,
            t.quadrant,
            t.created_at,
            t.archived_at,
            t.total_work_duration_minutes,
            (SELECT GROUP_CONCAT(json_object('id', tg.id, 'name', tg.name, 'color', tg.color))
             FROM tags tg
             INNER JOIN task_tags tt ON tg.id = tt.tag_id
             WHERE tt.task_id = t.id) as tags
         FROM tasks t
         WHERE t.status = 4 
           AND t.archived = 1
           AND date(t.archived_at) = date(?)
         ORDER BY t.archived_at DESC"
    )
    .bind(&date)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn calculate_work_days(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    start_time: String,
    end_time: String,
) -> Result<f64, String> {
    let pool = db.get_pool();
    let service = WorkDurationService::new(pool.clone());
    
    let start = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| e.to_string())?;
    let end = NaiveDateTime::parse_from_str(&end_time, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| e.to_string())?;
    
    service.calculate_work_days(start, end).await.map_err(|e| e.to_string())
}
