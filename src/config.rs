use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub replacements: Vec<Replacement>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Replacement {
    pub source_host: String,
    pub dest_host: String,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err(format!(
                "配置文件不存在: {}",
                config_path.display()
            ));
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("无法读取配置文件: {}", e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| format!("配置文件格式错误: {}", e))?;

        if config.replacements.is_empty() {
            return Err("配置文件中没有定义任何替换规则".to_string());
        }

        Ok(config)
    }

    pub fn get_config_path() -> Result<PathBuf, String> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| "无法获取用户主目录".to_string())?;

        let app_config_dir = home_dir.join(".curl-transformer");
        let config_file = app_config_dir.join("config.toml");

        Ok(config_file)
    }

    pub fn create_default_config() -> Result<PathBuf, String> {
        let config_path = Self::get_config_path()?;
        let config_dir = config_path.parent().unwrap();

        // 创建配置目录（如果不存在）
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .map_err(|e| format!("无法创建配置目录: {}", e))?;
            println!("✅ 已创建配置目录: {}", config_dir.display());
        }

        // 如果配置文件已存在，不覆盖
        if config_path.exists() {
            return Ok(config_path);
        }

        // 创建示例配置
        let example_config = Config {
            replacements: vec![
                Replacement {
                    source_host: "https://api.example.com".to_string(),
                    dest_host: "http://localhost:3000".to_string(),
                },
            ],
        };

        let toml_string = toml::to_string_pretty(&example_config)
            .map_err(|e| format!("无法序列化配置: {}", e))?;

        fs::write(&config_path, toml_string)
            .map_err(|e| format!("无法写入配置文件: {}", e))?;

        println!("✅ 已创建示例配置文件: {}", config_path.display());

        Ok(config_path)
    }
}
