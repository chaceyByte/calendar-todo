#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, Emitter};

mod database;
mod db_manager;
mod config;
mod tasks;
mod calendar;
mod tags;
mod work_duration;
mod work_hours;

use db_manager::{DatabaseManager, DatabaseState};
use config::{AppConfig, AutoBackupConfig};
use database::Database;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 初始化配置系统
            let config = AppConfig::load_or_default(app.handle())
                .expect("配置初始化失败");
            
            println!("📋 配置加载成功，版本: {}", config.version);
            println!("🎨 当前主题: {}", config.app.theme);
            
            // 获取数据库路径
            let db_path = config.get_db_path(app.handle())
                .expect("获取数据库路径失败")
                .to_string_lossy()
                .to_string();
            
            println!("💾 数据库路径: {}", db_path);
            
            // 初始化数据库（传统方式，为了兼容其他模块）
            let rt = tokio::runtime::Runtime::new().expect("创建 Tokio runtime 失败");
            let db = rt.block_on(async {
                let db = Database::new(&db_path)
                    .await
                    .expect("数据库连接失败");
                db.initialize_database()
                    .await
                    .expect("数据库初始化失败");
                println!("✅ 数据库初始化成功");
                db
            });
            
            // 创建数据库管理器（用于动态切换）
            let db_manager = Arc::new(DatabaseManager::new());
            
            // 初始化数据库管理器
            rt.block_on(async {
                if let Err(e) = db_manager.initialize(&db_path).await {
                    eprintln!("❌ 数据库管理器初始化失败: {}", e);
                }
            });
            
            // 将配置、数据库和数据库管理器放入应用状态
            app.manage(Arc::new(config));
            app.manage(Arc::new(db));  // 为了兼容现有模块
            app.manage(db_manager);
            
            println!("✅ 应用初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 基础命令
            greet,

            // 配置管理命令
            get_config,
            update_theme,
            update_db_path,
            update_auto_backup,
            
            // 数据库管理命令
            get_db_status,
            switch_database,
            validate_db_path,

            // 任务管理命令（来自 tasks 模块）
            tasks::get_task,
            tasks::get_recent_tasks,
            tasks::get_tasks_by_quadrant,
            tasks::get_tasks_by_status,
            tasks::get_all_tasks,
            tasks::create_task,
            tasks::update_task,
            tasks::delete_task,
            tasks::update_task_status,
            tasks::update_task_status_with_context,
            tasks::search_tasks,
            tasks::update_task_quadrant,
            tasks::archive_task,

            // 标签管理命令
            tags::get_tags,
            tags::create_tag,
            tags::update_tag,
            tags::delete_tag,
            tags::get_tag_usage_count,
            tags::get_task_tags,
            tags::add_tag_to_task,
            tags::remove_tag_from_task,

            // 日历管理命令
            calendar::get_calendar_events,
            calendar::get_day_work_records,
            calendar::get_holiday_config,
            calendar::get_holiday_config_by_date,
            calendar::update_holiday_config,
            calendar::delete_holiday_config,
            calendar::batch_update_holiday_configs,
            calendar::validate_date_for_type,
            calendar::preview_batch_operation,
            calendar::toggle_date_type,
            calendar::get_month_date_details,

            // 工作时长管理命令
            work_duration::create_work_record,
            work_duration::close_active_work_record,
            work_duration::get_task_work_records,
            work_duration::get_archived_tasks,
            work_duration::get_archived_tasks_by_date,
            work_duration::calculate_work_days,

            // 工作时段配置命令
            work_hours::get_work_hours_by_date,
            work_hours::get_default_work_hours,
            work_hours::update_default_work_hours,
            work_hours::update_work_hours,
            work_hours::delete_work_hours,
            work_hours::batch_update_work_hours,
            work_hours::get_month_work_hours,
            work_hours::get_work_hours_in_range,
            work_hours::validate_work_hours,

            // 数据库测试
            test_database_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ========== 基础命令 ==========

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! 欢迎使用四象限任务管理", name)
}

#[tauri::command]
async fn test_database_connection(
    db_manager: tauri::State<'_, Arc<DatabaseManager>>,
) -> Result<String, String> {
    match db_manager.is_ready().await {
        true => Ok("数据库连接正常".to_string()),
        false => Err("数据库未就绪".to_string()),
    }
}

// ========== 配置管理命令 ==========

/// 获取当前配置
#[tauri::command]
fn get_config(
    config: tauri::State<'_, Arc<AppConfig>>,
) -> Result<AppConfig, String> {
    // 返回配置的克隆
    Ok((**config).clone())
}

/// 更新主题设置
#[tauri::command]
fn update_theme(
    theme: String,
    config: tauri::State<'_, Arc<AppConfig>>,
    app_handle: tauri::AppHandle,
) -> Result<AppConfig, String> {
    // 需要可变访问，所以我们需要重新加载和保存
    let mut new_config = AppConfig::load(&app_handle)
        .map_err(|e| e.to_string())?;
    
    new_config.set_theme(&theme)
        .map_err(|e| e.to_string())?;
    
    new_config.save(&app_handle)
        .map_err(|e| e.to_string())?;
    
    // 更新状态
    // 注意：这里我们不能直接修改 Arc 中的内容，需要重新加载
    // 在实际应用中，可能需要使用 Mutex 或其他同步原语
    
    Ok(new_config)
}

/// 更新数据库路径
#[tauri::command]
async fn update_db_path(
    path: String,
    config: tauri::State<'_, Arc<AppConfig>>,
    app_handle: tauri::AppHandle,
) -> Result<AppConfig, String> {
    let mut new_config = AppConfig::load(&app_handle)
        .map_err(|e| e.to_string())?;
    
    new_config.set_db_path(&path)
        .map_err(|e| e.to_string())?;
    
    new_config.save(&app_handle)
        .map_err(|e| e.to_string())?;
    
    Ok(new_config)
}

/// 更新自动备份配置
#[tauri::command]
fn update_auto_backup(
    backup_config: AutoBackupConfig,
    config: tauri::State<'_, Arc<AppConfig>>,
    app_handle: tauri::AppHandle,
) -> Result<AppConfig, String> {
    let mut new_config = AppConfig::load(&app_handle)
        .map_err(|e| e.to_string())?;
    
    new_config.set_auto_backup(backup_config)
        .map_err(|e| e.to_string())?;
    
    new_config.save(&app_handle)
        .map_err(|e| e.to_string())?;
    
    Ok(new_config)
}

// ========== 数据库管理命令 ==========

/// 获取数据库状态
#[tauri::command]
async fn get_db_status(
    db_manager: tauri::State<'_, Arc<DatabaseManager>>,
) -> Result<DbStatus, String> {
    let state = db_manager.get_state().await;
    
    let status = match state {
        DatabaseState::Ready(_) => DbStatus {
            ready: true,
            switching: false,
            error: None,
        },
        DatabaseState::Switching => DbStatus {
            ready: false,
            switching: true,
            error: None,
        },
        DatabaseState::Uninitialized => DbStatus {
            ready: false,
            switching: false,
            error: Some("数据库未初始化".to_string()),
        },
        DatabaseState::Error(msg) => DbStatus {
            ready: false,
            switching: false,
            error: Some(msg),
        },
    };
    
    Ok(status)
}

/// 切换数据库
#[tauri::command]
async fn switch_database(
    path: String,
    db_manager: tauri::State<'_, Arc<DatabaseManager>>,
    app_handle: tauri::AppHandle,
) -> Result<AppConfig, String> {
    // 1. 先更新配置
    let mut config = AppConfig::load(&app_handle)
        .map_err(|e| format!("加载配置失败: {}", e))?;
    
    config.set_db_path(&path)
        .map_err(|e| format!("路径验证失败: {}", e))?;
    
    config.save(&app_handle)
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    // 2. 执行数据库切换
    db_manager.switch_database(&path).await
        .map_err(|e| e.to_string())?;
    
    // 3. 发送切换成功事件
    app_handle.emit("database-switched", &path)
        .map_err(|e| format!("发送事件失败: {}", e))?;
    
    // 4. 返回更新后的配置
    Ok(config)
}

/// 验证数据库路径
#[tauri::command]
fn validate_db_path(path: String) -> Result<PathValidationResult, String> {
    if path.is_empty() {
        return Ok(PathValidationResult {
            valid: true,
            message: "将使用默认路径".to_string(),
        });
    }
    
    let path_buf = std::path::PathBuf::from(&path);
    
    // 检查父目录
    if let Some(parent) = path_buf.parent() {
        if !parent.exists() {
            return Ok(PathValidationResult {
                valid: false,
                message: format!("目录不存在: {:?}", parent),
            });
        }
        
        // 检查写入权限
        let test_file = parent.join(".write_test");
        match std::fs::File::create(&test_file) {
            Ok(_) => {
                let _ = std::fs::remove_file(&test_file);
            }
            Err(e) => {
                return Ok(PathValidationResult {
                    valid: false,
                    message: format!("目录无写入权限: {}", e),
                });
            }
        }
    }
    
    // 检查文件是否存在
    if path_buf.exists() {
        Ok(PathValidationResult {
            valid: true,
            message: "将连接到现有数据库".to_string(),
        })
    } else {
        Ok(PathValidationResult {
            valid: true,
            message: "将创建新数据库".to_string(),
        })
    }
}

/// 数据库状态响应
#[derive(serde::Serialize)]
struct DbStatus {
    ready: bool,
    switching: bool,
    error: Option<String>,
}

/// 路径验证结果
#[derive(serde::Serialize)]
struct PathValidationResult {
    valid: bool,
    message: String,
}