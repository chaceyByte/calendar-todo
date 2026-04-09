use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use thiserror::Error;

/// 当前配置版本
const CURRENT_CONFIG_VERSION: &str = "1.0.0";
const CONFIG_FILE_NAME: &str = "config.json";

/// 配置错误类型
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("配置验证错误: {0}")]
    Validation(String),
}

/// 应用根配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 配置版本号，用于迁移
    pub version: String,
    /// 应用设置
    pub app: AppSettings,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 主题设置: "light", "dark", "system"
    pub theme: String,
    /// 数据库文件路径，空字符串表示使用默认路径
    pub db_path: String,
    /// 自动备份配置
    pub auto_backup: AutoBackupConfig,
}

/// 自动备份配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoBackupConfig {
    /// 是否启用自动备份
    pub enabled: bool,
    /// 备份频率: "1h", "6h", "12h", "24h", "7d"
    pub frequency: String,
    /// 最大保留快照数量
    pub max_snapshots: u32,
    /// 是否启用云同步
    pub cloud_sync: bool,
}

impl Default for AutoBackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency: "12h".to_string(),
            max_snapshots: 15,
            cloud_sync: false,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            db_path: String::new(), // 空表示使用默认路径
            auto_backup: AutoBackupConfig::default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            app: AppSettings::default(),
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    fn get_config_path(app_handle: &AppHandle) -> Result<PathBuf, ConfigError> {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .map_err(|e| ConfigError::Validation(format!("无法获取配置目录: {}", e)))?;
        
        // 确保配置目录存在
        std::fs::create_dir_all(&config_dir)?;
        
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }

    /// 从文件加载配置，如果不存在则创建默认配置
    pub fn load_or_default(app_handle: &AppHandle) -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path(app_handle)?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let mut config: serde_json::Value = serde_json::from_str(&content)?;
            
            // 执行配置迁移
            Self::migrate(&mut config);
            
            // 验证并返回
            let config: AppConfig = serde_json::from_value(config)?;
            config.validate()?;
            Ok(config)
        } else {
            // 创建默认配置并保存
            let config = Self::default();
            config.save(app_handle)?;
            Ok(config)
        }
    }

    /// 从文件加载配置
    pub fn load(app_handle: &AppHandle) -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path(app_handle)?;
        let content = std::fs::read_to_string(&config_path)?;
        let mut config: serde_json::Value = serde_json::from_str(&content)?;
        
        // 执行配置迁移
        Self::migrate(&mut config);
        
        let config: AppConfig = serde_json::from_value(config)?;
        config.validate()?;
        Ok(config)
    }

    /// 保存配置到文件
    pub fn save(&self, app_handle: &AppHandle) -> Result<(), ConfigError> {
        let config_path = Self::get_config_path(app_handle)?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// 获取数据库路径，如果未设置则返回默认路径
    pub fn get_db_path(&self, app_handle: &AppHandle) -> Result<PathBuf, ConfigError> {
        if self.app.db_path.is_empty() {
            // 使用默认路径
            let app_dir = app_handle
                .path()
                .app_data_dir()
                .map_err(|e| ConfigError::Validation(format!("无法获取数据目录: {}", e)))?;
            std::fs::create_dir_all(&app_dir)?;
            Ok(app_dir.join("tasks.db"))
        } else {
            Ok(PathBuf::from(&self.app.db_path))
        }
    }

    /// 验证配置有效性
    fn validate(&self) -> Result<(), ConfigError> {
        // 验证主题值
        if !matches!(self.app.theme.as_str(), "light" | "dark" | "system") {
            return Err(ConfigError::Validation(
                format!("无效的主题值: {}", self.app.theme)
            ));
        }

        // 验证备份频率
        if !matches!(
            self.app.auto_backup.frequency.as_str(),
            "1h" | "6h" | "12h" | "24h" | "7d"
        ) {
            return Err(ConfigError::Validation(
                format!("无效的备份频率: {}", self.app.auto_backup.frequency)
            ));
        }

        // 验证数据库路径（如果设置了）
        if !self.app.db_path.is_empty() {
            let path = PathBuf::from(&self.app.db_path);
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    return Err(ConfigError::Validation(
                        format!("数据库路径的父目录不存在: {:?}", parent)
                    ));
                }
            }
        }

        Ok(())
    }

    /// 配置迁移逻辑
    fn migrate(config: &mut serde_json::Value) {
        let current_version = config
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0");

        // 版本比较和迁移
        match current_version {
            "0.0.0" | "0.1.0" => {
                // 从旧版本迁移到 1.0.0
                Self::migrate_to_v1_0_0(config);
            }
            "1.0.0" => {
                // 当前版本，无需迁移
            }
            _ => {
                // 未知版本，尝试使用默认配置结构
                eprintln!("警告: 未知的配置版本 {}, 尝试兼容处理", current_version);
            }
        }
    }

    /// 迁移到 v1.0.0
    fn migrate_to_v1_0_0(config: &mut serde_json::Value) {
        eprintln!("执行配置迁移: 升级到 v1.0.0");

        // 确保 app 字段存在
        if config.get("app").is_none() {
            config["app"] = serde_json::json!({});
        }

        let app = &mut config["app"];

        // 迁移主题设置
        if app.get("theme").is_none() {
            // 尝试从旧配置中读取，或使用默认值
            app["theme"] = serde_json::json!("system");
        }

        // 迁移数据库路径
        if app.get("db_path").is_none() {
            app["db_path"] = serde_json::json!("");
        }

        // 迁移自动备份配置
        if app.get("auto_backup").is_none() {
            app["auto_backup"] = serde_json::json!({
                "enabled": false,
                "frequency": "12h",
                "max_snapshots": 15,
                "cloud_sync": false
            });
        } else {
            // 确保 auto_backup 的所有字段都存在
            let backup = &mut app["auto_backup"];
            if backup.get("enabled").is_none() {
                backup["enabled"] = serde_json::json!(false);
            }
            if backup.get("frequency").is_none() {
                backup["frequency"] = serde_json::json!("12h");
            }
            if backup.get("max_snapshots").is_none() {
                backup["max_snapshots"] = serde_json::json!(15);
            }
            if backup.get("cloud_sync").is_none() {
                backup["cloud_sync"] = serde_json::json!(false);
            }
        }

        // 更新版本号
        config["version"] = serde_json::json!(CURRENT_CONFIG_VERSION);
    }

    /// 更新主题设置
    pub fn set_theme(&mut self, theme: &str) -> Result<(), ConfigError> {
        if !matches!(theme, "light" | "dark" | "system") {
            return Err(ConfigError::Validation(
                format!("无效的主题值: {}", theme)
            ));
        }
        self.app.theme = theme.to_string();
        Ok(())
    }

    /// 更新数据库路径
    pub fn set_db_path(&mut self, path: &str) -> Result<(), ConfigError> {
        if !path.is_empty() {
            let path_buf = PathBuf::from(path);
            if let Some(parent) = path_buf.parent() {
                if !parent.exists() {
                    return Err(ConfigError::Validation(
                        format!("目录不存在: {:?}", parent)
                    ));
                }
            }
        }
        self.app.db_path = path.to_string();
        Ok(())
    }

    /// 更新自动备份配置
    pub fn set_auto_backup(&mut self, config: AutoBackupConfig) -> Result<(), ConfigError> {
        if !matches!(config.frequency.as_str(), "1h" | "6h" | "12h" | "24h" | "7d") {
            return Err(ConfigError::Validation(
                format!("无效的备份频率: {}", config.frequency)
            ));
        }
        self.app.auto_backup = config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.version, CURRENT_CONFIG_VERSION);
        assert_eq!(config.app.theme, "system");
        assert!(config.app.db_path.is_empty());
        assert!(!config.app.auto_backup.enabled);
    }

    #[test]
    fn test_validate_theme() {
        let mut config = AppConfig::default();
        
        // 有效值
        assert!(config.set_theme("light").is_ok());
        assert!(config.set_theme("dark").is_ok());
        assert!(config.set_theme("system").is_ok());
        
        // 无效值
        assert!(config.set_theme("invalid").is_err());
    }

    #[test]
    fn test_migrate_from_empty() {
        let mut config = serde_json::json!({});
        AppConfig::migrate(&mut config);
        
        assert_eq!(config["version"].as_str().unwrap(), CURRENT_CONFIG_VERSION);
        assert!(config["app"]["theme"].is_string());
        assert!(config["app"]["auto_backup"].is_object());
    }
}