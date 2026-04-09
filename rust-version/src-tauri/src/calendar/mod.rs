use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

use crate::database::Database;

#[cfg(test)]
mod tests;

/// 日期类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DateType {
    /// 正常工作日（周一至周五）
    Workday,
    /// 节假日/休假
    Holiday,
    /// 补班日（周末调整为工作日）
    Makeup,
    /// 周末休息日
    Weekend,
}

impl DateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DateType::Workday => "workday",
            DateType::Holiday => "holiday",
            DateType::Makeup => "makeup",
            DateType::Weekend => "weekend",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "workday" => Some(DateType::Workday),
            "holiday" => Some(DateType::Holiday),
            "makeup" => Some(DateType::Makeup),
            "weekend" => Some(DateType::Weekend),
            _ => None,
        }
    }
}

/// 节假日配置数据库模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HolidayConfig {
    pub id: i64,
    pub year: String,
    pub date: NaiveDate,
    #[serde(rename = "date_type")]
    pub r#type: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 创建/更新节假日配置请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHolidayConfigRequest {
    pub year: String,
    pub date: String,
    pub r#type: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// 批量更新节假日配置请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateHolidayRequest {
    pub dates: Vec<String>,
    pub r#type: String,
    pub name: Option<String>,
}

/// 日期详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DateDetail {
    pub date: String,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub day_of_week: i32,
    pub day_of_week_name: String,
    pub is_weekend: bool,
    pub date_type: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// 月份日历数据
#[derive(Debug, Serialize, Deserialize)]
pub struct MonthCalendarData {
    pub year: i32,
    pub month: i32,
    pub days: Vec<DayData>,
}

/// 单日数据
#[derive(Debug, Serialize, Deserialize)]
pub struct DayData {
    pub date: String,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub day_of_week: i32,
    pub day_of_week_name: String,
    pub is_current_month: bool,
    pub is_today: bool,
    pub is_weekend: bool,
    /// 日期类型: workday, holiday, makeup, weekend
    pub date_type: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tasks: Vec<CalendarEvent>,
}

/// 日历事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: i64,
    pub task_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub color: String,
    pub task_status: String,
    pub task_quadrant: i32,
}

/// 日期规则验证结果
#[derive(Debug, Serialize, Deserialize)]
pub struct DateValidationResult {
    pub date: String,
    pub is_valid: bool,
    pub can_be_holiday: bool,
    pub can_be_makeup: bool,
    pub reason: Option<String>,
}

/// 批量操作预览结果
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchPreviewResult {
    pub total_days: i32,
    pub workdays: i32,
    pub weekends: i32,
    pub holidays: i32,
    pub makeups: i32,
    pub invalid_dates: Vec<DateValidationResult>,
    pub preview: Vec<DateDetail>,
}

/// 日历服务
pub struct CalendarService {
    pool: SqlitePool,
}

impl CalendarService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取指定日期的详细工作记录（用于Day视图）
    pub async fn get_day_work_records(&self, date: &str) -> Result<Vec<DayWorkRecord>, sqlx::Error> {
        sqlx::query_as::<_, DayWorkRecord>(
            "SELECT 
                twr.id,
                twr.task_id,
                twr.start_time,
                twr.end_time,
                twr.duration_minutes,
                twr.record_type,
                t.title as task_title,
                t.description as task_description,
                t.status as task_status,
                t.quadrant as task_quadrant
             FROM task_work_records twr
             INNER JOIN tasks t ON twr.task_id = t.id
             WHERE date(twr.start_time) = ?
             ORDER BY twr.start_time"
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await
    }

    /// 获取指定月份的日历数据
    pub async fn get_calendar_events(&self, year: i32, month: i32) -> Result<MonthCalendarData, sqlx::Error> {
        // 获取指定月份的任务（基于任务计划时间）
        let tasks: Vec<TaskRow> = sqlx::query_as(
            "SELECT t.* FROM tasks t
             WHERE (t.start_at IS NOT NULL OR t.due_at IS NOT NULL)
               AND ((strftime('%Y', t.start_at) = ? AND strftime('%m', t.start_at) = ?)
                OR (strftime('%Y', t.due_at) = ? AND strftime('%m', t.due_at) = ?))
             ORDER BY COALESCE(t.start_at, t.due_at)"
        )
        .bind(year.to_string())
        .bind(format!("{:02}", month))
        .bind(year.to_string())
        .bind(format!("{:02}", month))
        .fetch_all(&self.pool)
        .await?;

        // 获取指定月份的工作记录（用于计算每天的活动任务）
        let work_records = self.get_work_records_for_month(year, month).await?;

        // 获取节假日配置
        let holiday_configs = self.get_holiday_configs_for_year(year).await?;
        let holiday_map: std::collections::HashMap<NaiveDate, &HolidayConfig> = 
            holiday_configs.iter().map(|h| (h.date, h)).collect();

        // 生成月份天数数据
        let start_date = NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| sqlx::Error::Decode("Invalid date".into()))?;
        
        let days_in_month = Self::days_in_month(year, month as u32);
        let today = chrono::Local::now().naive_local().date();
        
        let mut days = Vec::new();

        // 添加上个月的日期（填充日历开头）
        let first_day_weekday = start_date.weekday().num_days_from_monday() as i32;
        if first_day_weekday > 0 {
            let prev_month = if month == 1 { 12 } else { month - 1 };
            let prev_year = if month == 1 { year - 1 } else { year };
            let prev_month_days = Self::days_in_month(prev_year, prev_month as u32);
            
            for i in (0..first_day_weekday).rev() {
                let day = prev_month_days - i;
                let date = NaiveDate::from_ymd_opt(prev_year, prev_month as u32, day as u32)
                    .unwrap_or(start_date);
                days.push(self.create_day_data(date, year, month, today, &holiday_map, &tasks, &work_records, false));
            }
        }

        // 添加当前月的日期
        for day in 1..=days_in_month {
            let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
                .ok_or_else(|| sqlx::Error::Decode("Invalid date".into()))?;
            days.push(self.create_day_data(date, year, month, today, &holiday_map, &tasks, &work_records, true));
        }

        // 添加下个月的日期（填充日历结尾，保持6行）
        let remaining = 42 - days.len();
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        
        for day in 1..=remaining {
            let date = NaiveDate::from_ymd_opt(next_year, next_month as u32, day as u32)
                .unwrap_or(start_date);
            days.push(self.create_day_data(date, year, month, today, &holiday_map, &tasks, &work_records, false));
        }

        Ok(MonthCalendarData {
            year,
            month,
            days,
        })
    }

    /// 获取指定月份的工作记录
    async fn get_work_records_for_month(&self, year: i32, month: i32) -> Result<Vec<WorkRecordRow>, sqlx::Error> {
        let start_date = format!("{}-{:02}-01", year, month);
        let end_date = format!("{}-{:02}-{:02}", year, month, Self::days_in_month(year, month as u32));
        
        sqlx::query_as::<_, WorkRecordRow>(
            "SELECT 
                twr.id,
                twr.task_id,
                twr.start_time,
                twr.end_time,
                twr.record_type,
                t.title as task_title,
                t.description as task_description,
                t.status as task_status,
                t.quadrant as task_quadrant
             FROM task_work_records twr
             INNER JOIN tasks t ON twr.task_id = t.id
             WHERE date(twr.start_time) <= ? 
               AND (twr.end_time IS NULL OR date(twr.end_time) >= ?)
             ORDER BY twr.start_time"
        )
        .bind(&end_date)
        .bind(&start_date)
        .fetch_all(&self.pool)
        .await
    }

    /// 创建单日数据
    fn create_day_data(
        &self,
        date: NaiveDate,
        _current_year: i32,
        _current_month: i32,
        today: NaiveDate,
        holiday_map: &std::collections::HashMap<NaiveDate, &HolidayConfig>,
        tasks: &[TaskRow],
        work_records: &[WorkRecordRow],
        is_current_month: bool,
    ) -> DayData {
        let date_str = date.format("%Y-%m-%d").to_string();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        let is_weekend = day_of_week >= 5; // 周六(5)或周日(6)
        let is_today = date == today;

        // 确定日期类型
        let (date_type, name, description) = if let Some(config) = holiday_map.get(&date) {
            (config.r#type.clone(), config.name.clone(), config.description.clone())
        } else if is_weekend {
            ("weekend".to_string(), None, None)
        } else {
            ("workday".to_string(), None, None)
        };

        // 获取当天的活动任务（基于工作记录）
        let day_tasks = self.get_active_tasks_for_date(date, tasks, work_records);

        DayData {
            date: date_str,
            year: date.year(),
            month: date.month() as i32,
            day: date.day() as i32,
            day_of_week,
            day_of_week_name: Self::get_weekday_name(day_of_week),
            is_current_month,
            is_today,
            is_weekend,
            date_type,
            name,
            description,
            tasks: day_tasks,
        }
    }

    /// 获取指定日期的活动任务
    /// 基于工作记录表计算：如果某天有工作记录（start_time <= date <= end_time），则该任务在该天是活动的
    fn get_active_tasks_for_date(
        &self,
        date: NaiveDate,
        tasks: &[TaskRow],
        work_records: &[WorkRecordRow],
    ) -> Vec<CalendarEvent> {
        let mut active_tasks: Vec<CalendarEvent> = Vec::new();
        let mut processed_task_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();

        // 遍历工作记录，找出在指定日期有活动的任务
        for record in work_records {
            // 检查该工作记录是否覆盖指定日期
            let record_start_date = record.start_time.date();
            let record_end_date = record.end_time.map(|dt| dt.date()).unwrap_or_else(|| {
                // 如果 end_time 为空，表示任务仍在进行中，使用当前日期或一个很远的未来日期
                chrono::Local::now().naive_local().date()
            });

            // 如果指定日期在工作记录的时间范围内
            if date >= record_start_date && date <= record_end_date {
                // 避免重复添加同一任务
                if processed_task_ids.insert(record.task_id) {
                    // 查找任务详情
                    if let Some(task) = tasks.iter().find(|t| t.id == record.task_id) {
                        active_tasks.push(CalendarEvent {
                            id: task.id,
                            task_id: task.id,
                            title: task.title.clone(),
                            description: task.description.clone(),
                            start_time: DateTime::from_naive_utc_and_offset(
                                task.start_at.unwrap_or(record.start_time),
                                Utc
                            ),
                            end_time: task.due_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                            color: self.get_task_color(task.status),
                            task_status: self.get_status_text(task.status),
                            task_quadrant: task.quadrant,
                        });
                    } else {
                        // 如果任务不在tasks列表中（可能是跨月份的工作记录），使用工作记录中的信息
                        active_tasks.push(CalendarEvent {
                            id: record.task_id,
                            task_id: record.task_id,
                            title: record.task_title.clone(),
                            description: record.task_description.clone(),
                            start_time: DateTime::from_naive_utc_and_offset(record.start_time, Utc),
                            end_time: record.end_time.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                            color: self.get_task_color(record.task_status),
                            task_status: self.get_status_text(record.task_status),
                            task_quadrant: record.task_quadrant,
                        });
                    }
                }
            }
        }

        // 如果没有工作记录，回退到使用任务计划时间
        if active_tasks.is_empty() {
            active_tasks = tasks
                .iter()
                .filter(|task| {
                    let task_start_date = task.start_at.map(|dt| dt.date());
                    let task_end_date = task.due_at.map(|dt| dt.date());
                    
                    // 任务在某天活动的条件：
                    // 1. 任务开始日期 <= 指定日期
                    // 2. 任务截止日期 >= 指定日期 或 任务没有截止日期
                    match (task_start_date, task_end_date) {
                        (Some(start), Some(end)) => date >= start && date <= end,
                        (Some(start), None) => date >= start,
                        _ => false,
                    }
                })
                .map(|task| CalendarEvent {
                    id: task.id,
                    task_id: task.id,
                    title: task.title.clone(),
                    description: task.description.clone(),
                    start_time: DateTime::from_naive_utc_and_offset(
                        task.start_at.unwrap_or(task.due_at.unwrap()),
                        Utc
                    ),
                    end_time: task.due_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                    color: self.get_task_color(task.status),
                    task_status: self.get_status_text(task.status),
                    task_quadrant: task.quadrant,
                })
                .collect();
        }

        active_tasks
    }

    /// 获取指定年份的所有节假日配置
    pub async fn get_holiday_configs_for_year(&self, year: i32) -> Result<Vec<HolidayConfig>, sqlx::Error> {
        sqlx::query_as::<_, HolidayConfig>(
            "SELECT * FROM holiday_configs WHERE year = ? ORDER BY date"
        )
        .bind(year.to_string())
        .fetch_all(&self.pool)
        .await
    }

    /// 获取指定日期范围的节假日配置
    pub async fn get_holiday_configs_in_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HolidayConfig>, sqlx::Error> {
        sqlx::query_as::<_, HolidayConfig>(
            "SELECT * FROM holiday_configs WHERE date >= ? AND date <= ? ORDER BY date"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
    }

    /// 获取单个日期配置
    pub async fn get_holiday_config_by_date(&self, date: &str) -> Result<Option<HolidayConfig>, sqlx::Error> {
        sqlx::query_as::<_, HolidayConfig>(
            "SELECT * FROM holiday_configs WHERE date = ?"
        )
        .bind(date)
        .fetch_optional(&self.pool)
        .await
    }

    /// 更新或创建节假日配置
    pub async fn update_holiday_config(&self, req: UpdateHolidayConfigRequest) -> Result<HolidayConfig, sqlx::Error> {
        let date = NaiveDate::parse_from_str(&req.date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // 验证日期规则
        let validation = self.validate_date_for_type(&req.date, &req.r#type)?;
        if !validation.is_valid {
            return Err(sqlx::Error::Decode(
                validation.reason.unwrap_or_else(|| "日期验证失败".to_string()).into()
            ));
        }

        let now = Utc::now().naive_utc();

        // 插入或更新
        let id: i64 = sqlx::query(
            "INSERT INTO holiday_configs (year, date, type, name, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(year, date) DO UPDATE SET
             type = excluded.type,
             name = excluded.name,
             description = excluded.description,
             updated_at = excluded.updated_at
             RETURNING id"
        )
        .bind(&req.year)
        .bind(date)
        .bind(&req.r#type)
        .bind(&req.name)
        .bind(&req.description)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        sqlx::query_as::<_, HolidayConfig>(
            "SELECT * FROM holiday_configs WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    /// 删除节假日配置
    pub async fn delete_holiday_config(&self, date: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM holiday_configs WHERE date = ?")
            .bind(date)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 批量更新节假日配置
    pub async fn batch_update_holiday_configs(&self, req: BatchUpdateHolidayRequest) -> Result<Vec<HolidayConfig>, sqlx::Error> {
        let mut results = Vec::new();

        for date_str in &req.dates {
            // 验证日期
            let validation = self.validate_date_for_type(date_str, &req.r#type)?;
            if !validation.is_valid {
                continue; // 跳过无效日期
            }

            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let year = date.year().to_string();

            let update_req = UpdateHolidayConfigRequest {
                year,
                date: date_str.clone(),
                r#type: req.r#type.clone(),
                name: req.name.clone(),
                description: None,
            };

            match self.update_holiday_config(update_req).await {
                Ok(config) => results.push(config),
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    /// 验证日期是否可以设置为指定类型
    pub fn validate_date_for_type(&self, date_str: &str, date_type: &str) -> Result<DateValidationResult, sqlx::Error> {
        let date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => return Ok(DateValidationResult {
                date: date_str.to_string(),
                is_valid: false,
                can_be_holiday: false,
                can_be_makeup: false,
                reason: Some(format!("日期格式错误: {}", e)),
            }),
        };

        let day_of_week = date.weekday().num_days_from_monday() as i32;
        let is_weekend = day_of_week >= 5;

        match date_type {
            "holiday" => {
                // 休假规则：仅允许在周一至周五中设置休假
                if is_weekend {
                    Ok(DateValidationResult {
                        date: date_str.to_string(),
                        is_valid: false,
                        can_be_holiday: false,
                        can_be_makeup: false,
                        reason: Some("周末不能设置为节假日，周末本身就是休息日".to_string()),
                    })
                } else {
                    Ok(DateValidationResult {
                        date: date_str.to_string(),
                        is_valid: true,
                        can_be_holiday: true,
                        can_be_makeup: false,
                        reason: None,
                    })
                }
            }
            "makeup" => {
                // 补班规则：仅允许将周一至周五设置为补班日期
                // 实际上补班应该是将周末调整为工作日，所以这里需要修正逻辑
                // 补班应该是：在周末设置补班标记，表示这个周末需要上班
                if is_weekend {
                    Ok(DateValidationResult {
                        date: date_str.to_string(),
                        is_valid: true,
                        can_be_holiday: false,
                        can_be_makeup: true,
                        reason: None,
                    })
                } else {
                    Ok(DateValidationResult {
                        date: date_str.to_string(),
                        is_valid: false,
                        can_be_holiday: false,
                        can_be_makeup: false,
                        reason: Some("工作日不需要设置为补班，补班仅用于周末调整为工作日".to_string()),
                    })
                }
            }
            "workday" => {
                // 恢复为默认工作日
                Ok(DateValidationResult {
                    date: date_str.to_string(),
                    is_valid: true,
                    can_be_holiday: !is_weekend,
                    can_be_makeup: is_weekend,
                    reason: None,
                })
            }
            _ => Ok(DateValidationResult {
                date: date_str.to_string(),
                is_valid: false,
                can_be_holiday: false,
                can_be_makeup: false,
                reason: Some(format!("未知的日期类型: {}", date_type)),
            }),
        }
    }

    /// 批量验证日期范围
    pub async fn preview_batch_operation(
        &self,
        start_date: &str,
        end_date: &str,
        date_type: &str,
    ) -> Result<BatchPreviewResult, sqlx::Error> {
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let mut preview = Vec::new();
        let mut invalid_dates = Vec::new();
        let mut workdays = 0;
        let mut weekends = 0;
        let mut holidays = 0;
        let mut makeups = 0;

        let mut current = start;
        while current <= end {
            let date_str = current.format("%Y-%m-%d").to_string();
            let validation = self.validate_date_for_type(&date_str, date_type)?;

            let day_of_week = current.weekday().num_days_from_monday() as i32;
            let is_weekend = day_of_week >= 5;

            if validation.is_valid {
                match date_type {
                    "holiday" => holidays += 1,
                    "makeup" => makeups += 1,
                    "workday" => {
                        if is_weekend {
                            weekends += 1;
                        } else {
                            workdays += 1;
                        }
                    }
                    _ => {}
                }

                preview.push(DateDetail {
                    date: date_str.clone(),
                    year: current.year(),
                    month: current.month() as i32,
                    day: current.day() as i32,
                    day_of_week,
                    day_of_week_name: Self::get_weekday_name(day_of_week),
                    is_weekend,
                    date_type: date_type.to_string(),
                    name: None,
                    description: None,
                });
            } else {
                invalid_dates.push(validation);
                if is_weekend {
                    weekends += 1;
                } else {
                    workdays += 1;
                }
            }

            current = match current.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(BatchPreviewResult {
            total_days: preview.len() as i32 + invalid_dates.len() as i32,
            workdays,
            weekends,
            holidays,
            makeups,
            invalid_dates,
            preview,
        })
    }

    /// 切换日期类型（用于点击日历日期）
    pub async fn toggle_date_type(&self, date_str: &str) -> Result<HolidayConfig, sqlx::Error> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let year = date.year().to_string();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        let is_weekend = day_of_week >= 5;

        // 获取当前配置
        let current_config = self.get_holiday_config_by_date(date_str).await?;

        // 确定下一个类型
        let next_type = match current_config {
            Some(config) => {
                match config.r#type.as_str() {
                    "holiday" => {
                        // 节假日 -> 恢复默认
                        if is_weekend { "weekend" } else { "workday" }
                    }
                    "makeup" => {
                        // 补班 -> 恢复默认
                        if is_weekend { "weekend" } else { "workday" }
                    }
                    _ => {
                        // 默认 -> 根据规则切换
                        if is_weekend {
                            "makeup" // 周末 -> 补班
                        } else {
                            "holiday" // 工作日 -> 节假日
                        }
                    }
                }
            }
            None => {
                // 无配置 -> 根据规则切换
                if is_weekend {
                    "makeup" // 周末 -> 补班
                } else {
                    "holiday" // 工作日 -> 节假日
                }
            }
        };

        // 如果恢复为默认，删除配置
        if next_type == "workday" || next_type == "weekend" {
            self.delete_holiday_config(date_str).await?;
            
            // 返回一个虚拟的配置对象
            let now = Utc::now().naive_utc();
            return Ok(HolidayConfig {
                id: 0,
                year,
                date,
                r#type: next_type.to_string(),
                name: None,
                description: None,
                created_at: now,
                updated_at: now,
            });
        }

        // 否则更新配置
        let req = UpdateHolidayConfigRequest {
            year,
            date: date_str.to_string(),
            r#type: next_type.to_string(),
            name: None,
            description: None,
        };

        self.update_holiday_config(req).await
    }

    /// 获取指定月份的所有日期详情
    pub async fn get_month_date_details(&self, year: i32, month: i32) -> Result<Vec<DateDetail>, sqlx::Error> {
        let start_date = NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| sqlx::Error::Decode("Invalid date".into()))?;
        let end_date = Self::last_day_of_month(year, month as u32);

        let configs = self.get_holiday_configs_in_range(
            &start_date.format("%Y-%m-%d").to_string(),
            &end_date.format("%Y-%m-%d").to_string(),
        ).await?;

        let config_map: std::collections::HashMap<NaiveDate, &HolidayConfig> = 
            configs.iter().map(|c| (c.date, c)).collect();

        let mut details = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            let date_str = current.format("%Y-%m-%d").to_string();
            let day_of_week = current.weekday().num_days_from_monday() as i32;
            let is_weekend = day_of_week >= 5;

            let (date_type, name, description) = if let Some(config) = config_map.get(&current) {
                (config.r#type.clone(), config.name.clone(), config.description.clone())
            } else if is_weekend {
                ("weekend".to_string(), None, None)
            } else {
                ("workday".to_string(), None, None)
            };

            details.push(DateDetail {
                date: date_str,
                year: current.year(),
                month: current.month() as i32,
                day: current.day() as i32,
                day_of_week,
                day_of_week_name: Self::get_weekday_name(day_of_week),
                is_weekend,
                date_type,
                name,
                description,
            });

            current = match current.succ_opt() {
                Some(d) => d,
                None => break,
            };
        }

        Ok(details)
    }

    // 辅助方法

    fn days_in_month(year: i32, month: u32) -> i32 {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        
        let first_day_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        let last_day_current = first_day_next.pred_opt().unwrap();
        last_day_current.day() as i32
    }

    fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        
        let first_day_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        first_day_next.pred_opt().unwrap()
    }

    fn get_weekday_name(day_of_week: i32) -> String {
        match day_of_week {
            0 => "周一",
            1 => "周二",
            2 => "周三",
            3 => "周四",
            4 => "周五",
            5 => "周六",
            6 => "周日",
            _ => "未知",
        }.to_string()
    }

    fn get_task_color(&self, status: i32) -> String {
        match status {
            0 => "#909399", // 灰色 - Planning
            1 => "#409eff", // 蓝色 - InProgress
            2 => "#e6a23c", // 橙色 - Paused
            3 => "#67c23a", // 绿色 - Completed
            4 => "#f56c6c", // 红色 - Archived
            _ => "#909399",
        }
        .to_string()
    }

    fn get_status_text(&self, status: i32) -> String {
        match status {
            0 => "计划中".to_string(),
            1 => "进行中".to_string(),
            2 => "已暂停".to_string(),
            3 => "已完成".to_string(),
            4 => "已归档".to_string(),
            _ => "未知".to_string(),
        }
    }
}

// 工作记录行结构体（内部使用）
#[derive(Debug, Clone, FromRow)]
struct WorkRecordRow {
    pub id: i64,
    pub task_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub record_type: String,
    pub task_title: String,
    pub task_description: Option<String>,
    pub task_status: i32,
    pub task_quadrant: i32,
}

/// 日工作记录（用于Day视图展示）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayWorkRecord {
    pub id: i64,
    pub task_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration_minutes: Option<i64>,
    pub record_type: String,
    pub task_title: String,
    pub task_description: Option<String>,
    pub task_status: i32,
    pub task_quadrant: i32,
}

// 临时结构体用于查询任务
#[derive(Debug, Clone, FromRow)]
struct TaskRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub quadrant: i32,
    pub status: i32,
    pub start_at: Option<NaiveDateTime>,
    pub due_at: Option<NaiveDateTime>,
}

// ==================== Tauri Commands ====================

#[tauri::command]
pub async fn get_calendar_events(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    year: i32,
    month: i32,
) -> Result<MonthCalendarData, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.get_calendar_events(year, month).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_day_work_records(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<Vec<DayWorkRecord>, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.get_day_work_records(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_holiday_config(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    year: i32,
) -> Result<Vec<HolidayConfig>, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.get_holiday_configs_for_year(year).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_holiday_config_by_date(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<Option<HolidayConfig>, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.get_holiday_config_by_date(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_holiday_config(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: UpdateHolidayConfigRequest,
) -> Result<HolidayConfig, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.update_holiday_config(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_holiday_config(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<(), String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.delete_holiday_config(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_update_holiday_configs(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    request: BatchUpdateHolidayRequest,
) -> Result<Vec<HolidayConfig>, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.batch_update_holiday_configs(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_date_for_type(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
    date_type: String,
) -> Result<DateValidationResult, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.validate_date_for_type(&date, &date_type).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn preview_batch_operation(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    start_date: String,
    end_date: String,
    date_type: String,
) -> Result<BatchPreviewResult, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.preview_batch_operation(&start_date, &end_date, &date_type).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_date_type(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    date: String,
) -> Result<HolidayConfig, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.toggle_date_type(&date).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_month_date_details(
    db: tauri::State<'_, std::sync::Arc<Database>>,
    year: i32,
    month: i32,
) -> Result<Vec<DateDetail>, String> {
    let pool = db.get_pool();
    let service = CalendarService::new(pool.clone());
    
    service.get_month_date_details(year, month).await.map_err(|e| e.to_string())
}
