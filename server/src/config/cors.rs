use serde::Deserialize;

/// CORS 白名单条目
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CorsWhitelistItem {
    pub allow_origin: String,
    pub allow_headers: String,
    pub allow_methods: String,
    pub expose_headers: String,
    pub allow_credentials: bool,
}

/// CORS 配置，对应 config.yaml cors 节
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CorsConfig {
    /// 模式: allow-all / whitelist / strict-whitelist
    pub mode: String,
    pub whitelist: Vec<CorsWhitelistItem>,
}
