use serde::Deserialize;

/// JWT 配置，对应 config.yaml jwt 节
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct JwtConfig {
    /// 签名密钥
    pub signing_key: String,
    /// token 过期时间，如 "7d"、"24h"
    pub expires_time: String,
    /// token 缓冲时间（提前刷新），如 "1d"
    pub buffer_time: String,
    /// 签发者
    pub issuer: String,
}
