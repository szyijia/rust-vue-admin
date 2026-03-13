use config::{Config, File};
use std::path::Path;

use crate::config::AppConfig;

/// 配置文件相关常量，对应 Go 版本 core/internal/constant.go
const CONFIG_ENV: &str = "CONFIG_PATH";
const CONFIG_DEFAULT_FILE: &str = "config.yaml";

/// 获取配置文件路径，对应 Go 版本 core.getConfigPath()
///
/// 优先级（从高到低）：
/// 1. CONFIG_PATH 环境变量
/// 2. config.{env}.yaml — 根据 config.yaml 中 system.env 的值选择（如 local/development/production）
/// 3. config.yaml（默认）
///
/// 例如 system.env = "local" 时，会尝试加载 config.local.yaml，如果不存在则回退到 config.yaml
pub fn get_config_path() -> String {
    // 1. 最高优先级：CONFIG_PATH 环境变量
    if let Ok(env_path) = std::env::var(CONFIG_ENV) {
        if !env_path.is_empty() {
            println!(
                "您正在使用 {} 环境变量, config 的路径为 {}",
                CONFIG_ENV, env_path
            );
            return env_path;
        }
    }

    // 2. 尝试从默认 config.yaml 中读取 system.env，加载对应的环境配置文件
    if let Ok(content) = std::fs::read_to_string(CONFIG_DEFAULT_FILE) {
        if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(env) = yaml_value
                .get("system")
                .and_then(|s| s.get("env"))
                .and_then(|e| e.as_str())
            {
                if !env.is_empty() {
                    let env_config = format!("config.{}.yaml", env);
                    if Path::new(&env_config).exists() {
                        println!(
                            "您正在使用 {} 环境运行, config 的路径为 {}",
                            env, env_config
                        );
                        return env_config;
                    }
                    println!(
                        "配置文件 {} 不存在, 使用默认配置文件路径: {}",
                        env_config, CONFIG_DEFAULT_FILE
                    );
                }
            }
        }
    }

    // 3. 默认: config.yaml
    println!("config 的路径为 {}", CONFIG_DEFAULT_FILE);
    CONFIG_DEFAULT_FILE.to_string()
}

/// 加载配置文件，对应 Gin-Vue-Admin 的 core.Viper()
///
/// 加载顺序（后者覆盖前者）：
/// 1. 通过 get_config_path() 确定的配置文件
/// 2. 环境变量（前缀 RVA_，如 RVA_SYSTEM__ADDR=9090）
pub fn load_config() -> anyhow::Result<AppConfig> {
    let config_path = get_config_path();

    let cfg = Config::builder()
        // 基础配置文件
        .add_source(File::with_name(&config_path).required(true))
        // 环境变量覆盖（前缀 RVA，分隔符 __）
        .add_source(
            config::Environment::with_prefix("RVA")
                .separator("__")
                .try_parsing(true),
        )
        .build()?;

    let app_config: AppConfig = cfg.try_deserialize()?;

    Ok(app_config)
}
