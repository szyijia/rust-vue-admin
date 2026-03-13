use serde::Deserialize;

/// 邮件配置，对应 config.yaml email 节
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EmailConfig {
    pub to: String,
    pub port: u16,
    pub from: String,
    pub host: String,
    pub is_ssl: bool,
    pub secret: String,
    pub nickname: String,
}
