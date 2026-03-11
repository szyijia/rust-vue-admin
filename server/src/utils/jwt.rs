use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::JwtConfig;

/// JWT Claims 载荷，对应 Gin-Vue-Admin 的 request.CustomClaims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户 ID
    pub user_id: i64,
    /// 用户名
    pub username: String,
    /// 角色 ID
    pub role_id: i64,
    /// 角色名称
    pub role_name: String,
    /// 签发时间（Unix 时间戳）
    pub iat: i64,
    /// 过期时间（Unix 时间戳）
    pub exp: i64,
    /// 签发者
    pub iss: String,
    /// 缓冲期截止时间（用于判断是否需要刷新 token）
    pub buffer_time: i64,
}

/// Token 生成结果
#[derive(Debug, Serialize)]
pub struct TokenResult {
    pub token: String,
    pub expires_at: i64,
}

/// 生成 JWT Token，对应 Gin-Vue-Admin 的 utils.CreateToken()
pub fn create_token(
    user_id: i64,
    username: &str,
    role_id: i64,
    role_name: &str,
    cfg: &JwtConfig,
) -> anyhow::Result<TokenResult> {
    let now = Utc::now();
    let expires_duration = parse_duration(&cfg.expires_time)?;
    let buffer_duration = parse_duration(&cfg.buffer_time)?;

    let exp = (now + expires_duration).timestamp();
    let buffer_time = (now + buffer_duration).timestamp();

    let claims = Claims {
        user_id,
        username: username.to_string(),
        role_id,
        role_name: role_name.to_string(),
        iat: now.timestamp(),
        exp,
        iss: cfg.issuer.clone(),
        buffer_time,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.signing_key.as_bytes()),
    )?;

    Ok(TokenResult {
        token,
        expires_at: exp,
    })
}

/// 解析并验证 JWT Token，对应 Gin-Vue-Admin 的 utils.ParseToken()
pub fn parse_token(token: &str, cfg: &JwtConfig) -> anyhow::Result<Claims> {
    let mut validation = Validation::default();
    validation.set_issuer(&[&cfg.issuer]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.signing_key.as_bytes()),
        &validation,
    )
    .map_err(|e| anyhow::anyhow!("Token 解析失败: {}", e))?;

    Ok(token_data.claims)
}

/// 判断 token 是否在缓冲期内（需要刷新）
pub fn is_in_buffer_time(claims: &Claims) -> bool {
    let now = Utc::now().timestamp();
    now > claims.buffer_time && now < claims.exp
}

/// 解析时间字符串，支持 "7d"、"24h"、"30m"、"3600s" 格式
fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    if s.is_empty() {
        return Ok(Duration::hours(24));
    }

    let (num_str, unit) = s.split_at(s.len() - 1);
    let num: i64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("无效的时间格式: {}", s))?;

    match unit {
        "d" => Ok(Duration::days(num)),
        "h" => Ok(Duration::hours(num)),
        "m" => Ok(Duration::minutes(num)),
        "s" => Ok(Duration::seconds(num)),
        _ => Err(anyhow::anyhow!(
            "不支持的时间单位: {}，请使用 d/h/m/s",
            unit
        )),
    }
}
