pub mod captcha;
pub mod cors;
pub mod database;
pub mod email;
pub mod jwt;
pub mod log;
pub mod redis;
pub mod system;

pub use captcha::CaptchaConfig;
pub use cors::CorsConfig;
pub use database::{MysqlConfig, PgsqlConfig, SqliteConfig};
pub use email::EmailConfig;
pub use jwt::JwtConfig;
pub use log::LogConfig;
pub use redis::RedisConfig;
pub use system::SystemConfig;

use serde::Deserialize;

/// 全局配置结构体，对应 config.yaml 顶层结构
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub jwt: JwtConfig,
    pub log: LogConfig,
    pub redis: RedisConfig,
    pub email: EmailConfig,
    pub system: SystemConfig,
    pub captcha: CaptchaConfig,
    pub mysql: MysqlConfig,
    pub pgsql: PgsqlConfig,
    pub sqlite: SqliteConfig,
    pub cors: CorsConfig,
}
