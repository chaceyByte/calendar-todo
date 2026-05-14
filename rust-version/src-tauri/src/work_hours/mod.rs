use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

use crate::database::Database;

/// 工作时长配置
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkHoursConfig {
    pub id: i64,
    pub date: NaiveDate,
    pub morning_start_time: String,
    pub morning_end_time: String,
    pub afternoon_start_time: String,
    pub afternoon_end_time: String,
    pub total_work_minutes: i64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 默认工作时长配置
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DefaultWorkHours {
    pub id: i64,
    pub morning_start_time: String,
    pub morning_end_time: String,
    pub afternoon_start_time: String,
    pub afternoon_end_time: String,
    pub total_work_minutes: i64,
    pub updated_at: DateTime<Utc>,
}

/// 工作时段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSession {
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i64,
}

/// 工作时长详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkHoursDetail {
    pub date: String,
    pub morning_session: WorkSession,
    pub afternoon_session: WorkSession,
    pub total_hours: f64,
    pub total_minutes: i64,
    pub is_custom: bool,
    pub description: Option<String>,
}

/// 创建/更新工作时长配置请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkHoursRequest {
    pub date: String,
    pub morning_start_time: String,
    pub morning_end_time: String,
    pub afternoon_start_time: String,
    pub afternoon_end_time: String,
    pub description: Option<String>,
}

/// 更新默认工作时长请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDefaultWorkHoursRequest {
    pub morning_start_time: String,
    pub morning_end_time: String,
    pub afternoon_start_time: String,
    pub afternoon_end_time: String,
}

/// 批量更新工作时长请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateWorkHoursRequest {
    pub dates: Vec<String>,
    pub morning_start_time: String,
    pub morning_end_time: String,
    pub afternoon_start_time: String,
    pub afternoon_end_time: String,
    pub description: Option<String>,
}

/// 工作时长验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkHoursValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub total_minutes: i64,
}

/// 月份工作时长配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthWorkHoursConfig {
    pub year: i32,
    pub month: i32,
    pub configs: Vec<WorkHoursDetail>,
}

/// 工作时长配置服务
pub struct WorkHoursService {
    pool: SqlitePool,
}

impl WorkHoursService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 计算工作时长（分钟）
    fn calculate_work_minutes(
        morning_start: &str,
        morning_end: &str,
        afternoon_start: &str,
        afternoon_end: &str,
    ) -> Result<i64, String> {
        let morning_start = NaiveTime::parse_from_str(morning_start, "%H:%M")
            .map_err(|_| "上午开始时间格式错误".to_string())?;
        let morning_end = NaiveTime::parse_from_str(morning_end, "%H:%M")
            .map_err(|_| "上午结束时间格式错误".to_string())?;
        let afternoon_start = NaiveTime::parse_from_str(afternoon_start, "%H:%M")
            .map_err(|_| "下午开始时间格式错误".to_string())?;
        let afternoon_end = NaiveTime::parse_from_str(afternoon_end, "%H:%M")
            .map_err(|_| "下午结束时间格式错误".to_string())?;

        // 验证时间顺序
        if morning_start >= morning_end {
            return Err("上午开始时间必须早于结束时间".to_string());
        }
        if afternoon_start >= afternoon_end {
            return Err("下午开始时间必须早于结束时间".to_string());
        }
        if morning_end >= afternoon_start {
            return Err("上午结束时间必须早于下午开始时间".to_string());
        }

        let morning_minutes = (morning_end - morning_start).num_minutes();
        let afternoon_minutes = (afternoon_end - afternoon_start).num_minutes();

        Ok(morning_minutes + afternoon_minutes)
    }

    /// 验证工作时长配置
    pub fn validate_work_hours(
        morning_start: &str,
        morning_end: &str,
        afternoon_start: &str,
        afternoon_end: &str,
    ) -> WorkHoursValidationResult {
        match Self::calculate_work_minutes(
            morning_start,
            morning_end,
            afternoon_start,
            afternoon_end,
        ) {
            Ok(total_minutes) => WorkHoursValidationResult {
                is_valid: true,
                error_message: None,
                total_minutes,
            },
            Err(error_message) => WorkHoursValidationResult {
                is_valid: false,
                error_message: Some(error_message),
                total_minutes: 0,
            },
        }
    }

    /// 获取指定日期的工作时长配置
    pub async fn get_work_hours_by_date(
        &self,
        date: &str,
    ) -> Result<WorkHoursDetail, sqlx::Error> {
        let date_obj = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // 先查询特定日期的配置
        let config: Option<WorkHoursConfig> = sqlx::query_as(
            "SELECT * FROM work_hours_config WHERE date = ?"
        )
        .bind(date_obj)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(config) = config {
            let morning_duration = Self::calculate_work_minutes(
                &config.morning_start_time,
                &config.morning_end_time,
                &config.afternoon_start_time,
                &config.afternoon_end_time,
            ).unwrap_or(0);

            Ok(WorkHoursDetail {
                date: date.to_string(),
                morning_session: WorkSession {
                    start_time: config.morning_start_time.clone(),
                    end_time: config.morning_end_time.clone(),
                    duration_minutes: morning_duration,
                },
                afternoon_session: WorkSession {
                    start_time: config.afternoon_start_time.clone(),
                    end_time: config.afternoon_end_time.clone(),
                    duration_minutes: config.total_work_minutes - morning_duration,
                },
                total_hours: config.total_work_minutes as f64 / 60.0,
                total_minutes: config.total_work_minutes,
                is_custom: true,
                description: config.description,
            })
        } else {
            // 返回默认配置
            let default = self.get_default_work_hours().await?;
            let morning_duration = Self::calculate_work_minutes(
                &default.morning_start_time,
                &default.morning_end_time,
                &default.afternoon_start_time,
                &default.afternoon_end_time,
            ).unwrap_or(210); // 默认上午 3.5 小时

            Ok(WorkHoursDetail {
                date: date.to_string(),
                morning_session: WorkSession {
                    start_time: default.morning_start_time.clone(),
                    end_time: default.morning_end_time.clone(),
                    duration_minutes: morning_duration,
                },
                afternoon_session: WorkSession {
                    start_time: default.afternoon_start_time.clone(),
                    end_time: default.afternoon_end_time.clone(),
                    duration_minutes: default.total_work_minutes - morning_duration,
                },
                total_hours: default.total_work_minutes as f64 / 60.0,
                total_minutes: default.total_work_minutes,
                is_custom: false,
                description: None,
            })
        }
    }

    /// 获取默认工作时长配置
    pub async fn get_default_work_hours(&self) -> Result<DefaultWorkHours, sqlx::Error> {
        let default: Option<DefaultWorkHours> = sqlx::query_as(
            "SELECT * FROM default_work_hours WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(default) = default {
            Ok(default)
        } else {
            // 如果没有默认值，创建一条
            sqlx::query(
                "INSERT INTO default_work_hours (id) VALUES (1)"
            )
            .execute(&self.pool)
            .await?;

            sqlx::query_as(
                "SELECT * FROM default_work_hours WHERE id = 1"
            )
            .fetch_one(&self.pool)
            .await
        }
    }

    /// 更新默认工作时长配置
    pub async fn update_default_work_hours(
        &self,
        req: UpdateDefaultWorkHoursRequest,
    ) -> Result<DefaultWorkHours, sqlx::Error> {
        // 验证时间
        let validation = Self::validate_work_hours(
            &req.morning_start_time,
            &req.morning_end_time,
            &req.afternoon_start_time,
            &req.afternoon_end_time,
        );

        if !validation.is_valid {
            return Err(sqlx::Error::Decode(
                validation.error_message.unwrap_or_else(|| "时间验证失败".to_string()).into()
            ));
        }

        let now = Utc::now();

        sqlx::query(
            "UPDATE default_work_hours SET 
             morning_start_time = ?,
             morning_end_time = ?,
             afternoon_start_time = ?,
             afternoon_end_time = ?,
             total_work_minutes = ?,
             updated_at = ?
             WHERE id = 1"
        )
        .bind(&req.morning_start_time)
        .bind(&req.morning_end_time)
        .bind(&req.afternoon_start_time)
        .bind(&req.afternoon_end_time)
        .bind(validation.total_minutes)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.get_default_work_hours().await
    }

    /// 创建或更新指定日期的工作时长配置
    pub async fn update_work_hours(
        &self,
        req: UpdateWorkHoursRequest,
    ) -> Result<WorkHoursConfig, sqlx::Error> {
        let date = NaiveDate::parse_from_str(&req.date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // 验证时间
        let validation = Self::validate_work_hours(
            &req.morning_start_time,
            &req.morning_end_time,
            &req.afternoon_start_time,
            &req.afternoon_end_time,
        );

        if !validation.is_valid {
            return Err(sqlx::Error::Decode(
                validation.error_message.unwrap_or_else(|| "时间验证失败".to_string()).into()
            ));
        }

        let now = Utc::now();

        // 插入或更新
        let id: i64 = sqlx::query(
            "INSERT INTO work_hours_config 
             (date, morning_start_time, morning_end_time, afternoon_start_time, afternoon_end_time, total_work_minutes, description, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(date) DO UPDATE SET
             morning_start_time = excluded.morning_start_time,
             morning_end_time = excluded.morning_end_time,
             afternoon_start_time = excluded.afternoon_start_time,
             afternoon_end_time = excluded.afternoon_end_time,
             total_work_minutes = excluded.total_work_minutes,
             description = excluded.description,
             updated_at = excluded.updated_at
             RETURNING id"
        )
        .bind(date)
        .bind(&req.morning_start_time)
        .bind(&req.morning_end_time)
        .bind(&req.afternoon_start_time)
        .bind(&req.afternoon_end_time)
        .bind(validation.total_minutes)
        .bind(&req.description)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        sqlx::query_as::<_, WorkHoursConfig>(
            "SELECT * FROM work_hours_config WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    /// 删除指定日期的工作时长配置（恢复为默认）
    pub async fn delete_work_hours(&self, date: &str) -> Result<(), sqlx::Error> {
        let date_obj = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        sqlx::query("DELETE FROM work_hours_config WHERE date = ?")
            .bind(date_obj)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 批量更新工作时长配置
    pub async fn batch_update_work_hours(
        &self,
        req: BatchUpdateWorkHoursRequest,
    ) -> Result<Vec<WorkHoursConfig>, sqlx::Error> {
        let mut results = Vec::new();

        for date_str in &req.dates {
            let update_req = UpdateWorkHoursRequest {
                date: date_str.clone(),
                morning_start_time: req.morning_start_time.clone(),
                morning_end_time: req.morning_end_time.clone(),
                afternoon_start_time: req.afternoon_start_time.clone(),
                afternoon_end_time: req.afternoon_end_time.clone(),
                description: req.description.clone(),
            };

            match self.update_work_hours(update_req).await {
                Ok(config) => results.push(config),
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    /// 获取指定月份的所有工作时长配置
    pub async fn get_month_work_hours(
        &self,
        year: i32,
        month: i32,
    ) -> Result<MonthWorkHoursConfig, sqlx::Error> {
        let start_date = NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| sqlx::Error::Decode("Invalid date".into()))?;
        
        let end_date = Self::last_day_of_month(year, month as u32);

        // 获取该月所有自定义配置
        let configs: Vec<WorkHoursConfig> = sqlx::query_as(
            "SELECT * FROM work_hours_config WHERE date >= ? AND date <= ? ORDER BY date"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        let config_map: std::collections::HashMap<NaiveDate, &WorkHoursConfig> = 
            configs.iter().map(|c| (c.date, c)).collect();

        // 获取默认配置
        let default = self.get_default_work_hours().await?;

        let mut details = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            let date_str = current.format("%Y-%m-%d").to_string();

            let detail = if let Some(config) = config_map.get(&current) {
                let morning_duration = Self::calculate_work_minutes(
                    &config.morning_start_time,
                    &config.morning_end_time,
                    &config.afternoon_start_time,
                    &config.afternoon_end_time,
                ).unwrap_or(0);

                WorkHoursDetail {
                    date: date_str,
                    morning_session: WorkSession {
                        start_time: config.morning_start_time.clone(),
                        end_time: config.morning_end_time.clone(),
                        duration_minutes: morning_duration,
                    },
                    afternoon_session: WorkSession {
                        start_time: config.afternoon_start_time.clone(),
                        end_time: config.afternoon_end_time.clone(),
                        duration_minutes: config.total_work_minutes - morning_duration,
                    },
                    total_hours: config.total_work_minutes as f64 / 60.0,
                    total_minutes: config.total_work_minutes,
                    is_custom: true,
                    description: config.description.clone(),
                }
            } else {
                let morning_duration = Self::calculate_work_minutes(
                    &default.morning_start_time,
                    &default.morning_end_time,
                    &default.afternoon_start_time,
                    &default.afternoon_end_time,
                ).unwrap_or(210);

                WorkHoursDetail {
                    date: date_str,
                    morning_session: WorkSession {
                        start_time: default.morning_start_time.clone(),
                        end_time: default.morning_end_time.clone(),
                        duration_minutes: morning_duration,
                    },
                    afternoon_session: WorkSession {
                        start_time: default.afternoon_start_time.clone(),
                        end_time: default.afternoon_end_time.clone(),
                        duration_minutes: default.total_work_minutes - morning_duration,
                    },
                    total_hours: default.total_work_minutes as f64 / 60.0,
                    total_minutes: default.total_work_minutes,
                    is_custom: false,
                    description: None,
                }
            };

            details.push(detail);

            current = match current.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(MonthWorkHoursConfig {
            year,
            month,
            configs: details,
        })
    }

    /// 获取指定日期范围的工作时长配置
    pub async fn get_work_hours_in_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<WorkHoursDetail>, sqlx::Error> {
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // 获取范围内的自定义配置
        let configs: Vec<WorkHoursConfig> = sqlx::query_as(
            "SELECT * FROM work_hours_config WHERE date >= ? AND date <= ? ORDER BY date"
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        let config_map: std::collections::HashMap<NaiveDate, &WorkHoursConfig> = 
            configs.iter().map(|c| (c.date, c)).collect();

        // 获取默认配置
        let default = self.get_default_work_hours().await?;

        let mut details = Vec::new();
        let mut current = start;

        while current <= end {
            let date_str = current.format("%Y-%m-%d").to_string();

            let detail = if let Some(config) = config_map.get(&current) {
                let morning_duration = Self::calculate_work_minutes(
                    &config.morning_start_time,
                    &config.morning_end_time,
                    &config.afternoon_start_time,
                    &config.afternoon_end_time,
                ).unwrap_or(0);

                WorkHoursDetail {
                    date: date_str,
                    morning_session: WorkSession {
                        start_time: config.morning_start_time.clone(),
                        end_time: config.morning_end_time.clone(),
                        duration_minutes: morning_duration,
                    },
                    afternoon_session: WorkSession {
                        start_time: config.afternoon_start_time.clone(),
                        end_time: config.afternoon_end_time.clone(),
                        duration_minutes: config.total_work_minutes - morning_duration,
                    },
                    total_hours: config.total_work_minutes as f64 / 60.0,
                    total_minutes: config.total_work_minutes,
                    is_custom: true,
                    description: config.description.clone(),
                }
            } else {
                let morning_duration = Self::calculate_work_minutes(
                    &default.morning_start_time,
                    &default.morning_end_time,
                    &default.afternoon_start_time,
                    &default.afternoon_end_time,
                ).unwrap_or(210);

                WorkHoursDetail {
                    date: date_str,
                    morning_session: WorkSession {
                        start_time: default.morning_start_time.clone(),
                        end_time: default.morning_end_time.clone(),
                        duration_minutes: morning_duration,
                    },
                    afternoon_session: WorkSession {
                        start_time: default.afternoon_start_time.clone(),
                        end_time: default.afternoon_end_time.clone(),
                        duration_minutes: default.total_work_minutes - morning_duration,
                    },
                    total_hours: default.total_work_minutes as f64 / 60.0,
                    total_minutes: default.total_work_minutes,
                    is_custom: false,
                    description: None,
                }
            };

            details.push(detail);

            current = match current.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(details)
    }

    fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        
        let first_day_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        first_day_next.pred_opt().unwrap()
    }
}

// ==================== Tauri Commands ====================

#[tauri::command]
pub async fn get_work_hours_by_date(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<WorkHoursDetail, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.get_work_hours_by_date(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_default_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
) -> Result<DefaultWorkHours, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.get_default_work_hours().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_default_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: UpdateDefaultWorkHoursRequest,
) -> Result<DefaultWorkHours, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.update_default_work_hours(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: UpdateWorkHoursRequest,
) -> Result<WorkHoursConfig, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.update_work_hours(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<(), String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.delete_work_hours(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_update_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: BatchUpdateWorkHoursRequest,
) -> Result<Vec<WorkHoursConfig>, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.batch_update_work_hours(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_month_work_hours(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    year: i32,
    month: i32,
) -> Result<MonthWorkHoursConfig, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.get_month_work_hours(year, month).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_work_hours_in_range(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    start_date: String,
    end_date: String,
) -> Result<Vec<WorkHoursDetail>, String> {
    let pool = db.get_pool();
    let service = WorkHoursService::new(pool.clone());
    
    service.get_work_hours_in_range(&start_date, &end_date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn validate_work_hours(
    morning_start_time: String,
    morning_end_time: String,
    afternoon_start_time: String,
    afternoon_end_time: String,
) -> WorkHoursValidationResult {
    WorkHoursService::validate_work_hours(
        &morning_start_time,
        &morning_end_time,
        &afternoon_start_time,
        &afternoon_end_time,
    )
}
