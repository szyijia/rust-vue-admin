use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    global::{ApiResponse, AppState},
    utils::jwt,
};

/// JWT 认证中间件，对应 Gin-Vue-Admin 的 middleware.JWTAuth()
///
/// 从请求头 Authorization: Bearer <token> 中提取并验证 JWT
/// 验证通过后将 Claims 注入到请求扩展中，供后续 handler 使用
pub async fn jwt_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, axum::response::Response> {
    // 提取 Authorization 头
    let token = extract_bearer_token(&req).ok_or_else(|| {
        let resp: ApiResponse<()> = ApiResponse::unauthorized("未提供认证 Token");
        resp.into_http_response()
    })?;

    // 解析并验证 token
    let claims = jwt::parse_token(token, &state.config.jwt).map_err(|e| {
        let resp: ApiResponse<()> = ApiResponse::unauthorized(&format!("Token 无效: {}", e));
        resp.into_http_response()
    })?;

    // 将 Claims 注入请求扩展，handler 可通过 Extension<Claims> 获取
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// 从请求头中提取 Bearer Token
/// 支持两种方式：
/// 1. Authorization: Bearer <token>（标准方式）
/// 2. x-token: <token>（Gin-Vue-Admin 前端方式）
fn extract_bearer_token(req: &Request) -> Option<&str> {
    // 优先从 Authorization: Bearer 提取
    if let Some(token) = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        return Some(token);
    }

// 其次从 x-token 头提取（Gin-Vue-Admin 前端使用）
    req.headers()
        .get("x-token")
        .and_then(|v| v.to_str().ok())
}
