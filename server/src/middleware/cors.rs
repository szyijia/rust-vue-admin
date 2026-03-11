use axum::http::{HeaderName, HeaderValue, Method};
use std::str::FromStr;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders};

use crate::config::CorsConfig;

/// 根据配置构建 CORS 中间件层
/// 对应 Gin-Vue-Admin 的 middleware.Cors() / middleware.CorsWhitelist()
pub fn build_cors_layer(cfg: &CorsConfig) -> CorsLayer {
    match cfg.mode.as_str() {
        // 允许所有来源（开发环境使用）
        "allow-all" => CorsLayer::permissive(),

        // 白名单模式：允许白名单中的来源，但不严格限制
        "whitelist" | "strict-whitelist" => build_whitelist_cors(cfg),

        // 默认：严格模式
        _ => build_whitelist_cors(cfg),
    }
}

/// 构建白名单 CORS 层
fn build_whitelist_cors(cfg: &CorsConfig) -> CorsLayer {
    if cfg.whitelist.is_empty() {
        // 白名单为空时，拒绝所有跨域请求
        return CorsLayer::new();
    }

    // 收集所有允许的来源
    let origins: Vec<HeaderValue> = cfg
        .whitelist
        .iter()
        .filter_map(|item| HeaderValue::from_str(&item.allow_origin).ok())
        .collect();

    // 收集所有允许的 Header（取第一条配置，通常各条目一致）
    let first = &cfg.whitelist[0];

    let allow_headers: Vec<HeaderName> = first
        .allow_headers
        .split(',')
        .filter_map(|h| HeaderName::from_str(h.trim()).ok())
        .collect();

    let allow_methods: Vec<Method> = first
        .allow_methods
        .split(',')
        .filter_map(|m| Method::from_str(m.trim()).ok())
        .collect();

    let expose_headers: Vec<HeaderName> = first
        .expose_headers
        .split(',')
        .filter_map(|h| HeaderName::from_str(h.trim()).ok())
        .collect();

    let mut layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_headers(AllowHeaders::list(allow_headers))
        .allow_methods(AllowMethods::list(allow_methods))
        .expose_headers(ExposeHeaders::list(expose_headers));

    if first.allow_credentials {
        layer = layer.allow_credentials(true);
    }

    layer
}
