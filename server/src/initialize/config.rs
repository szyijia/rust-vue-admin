use config::{Config, File};

use crate::config::AppConfig;

/// 加载配置文件，对应 Gin-Vue-Admin 的 core.Viper()
///
/// 加载顺序（后者覆盖前者）：
/// 1. config.yaml（默认配置）
/// 2. config.{env}.yaml（环境特定配置，可选）
/// 3. 环境变量（前缀 RVA_，如 RVA_SYSTEM__ADDR=9090）
pub fn load_config() -> anyhow::Result<AppConfig> {
    // 确定配置文件路径（支持通过 CONFIG_PATH 环境变量指定）
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());

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
