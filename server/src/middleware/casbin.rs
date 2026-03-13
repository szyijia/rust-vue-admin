use axum::{
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use casbin::CoreApi;
use tracing::warn;

use crate::{global::{response::ApiResponse, AppState}, utils::jwt::Claims};

/// Casbin 权限鉴权中间件
/// 在 JWT 认证中间件之后执行，从请求扩展中取出 Claims，
/// 检查该角色是否有权限访问当前路径+方法
pub async fn casbin_auth(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    // 如果 Casbin 未初始化（无数据库），直接放行
    let Some(enforcer) = state.try_get_enforcer() else {
        return next.run(req).await.into_response();
    };

    // 从请求扩展中获取 JWT Claims（由 jwt_auth 中间件注入）
    let claims = req.extensions().get::<Claims>().cloned();
    let Some(claims) = claims else {
        return ApiResponse::<()>::unauthorized("未提供认证信息").into_response();
    };

    let path = req.uri().path().to_string();
    let method = req.method().clone();
    let method_str = method_to_str(&method);

    // 检查权限（与 gin-vue-admin 对齐，超管也需要在 casbin_rules 中有策略记录）
    let role_id_str = claims.role_id.to_string();
    let e = enforcer.read().await;
    match e.enforce(vec![role_id_str, path.clone(), method_str.to_string()]) {
        Ok(true) => {
            drop(e);
            next.run(req).await.into_response()
        }
        Ok(false) => {
            warn!("权限不足: role_id={}, path={}, method={}", claims.role_id, path, method_str);
            (
                StatusCode::FORBIDDEN,
                ApiResponse::<()>::forbidden("权限不足，无法访问该接口"),
            )
                .into_response()
        }
        Err(err) => {
            warn!("Casbin enforce 错误: {}", err);
            next.run(req).await.into_response()
        }
    }
}

fn method_to_str(method: &Method) -> &str {
    match *method {
        Method::GET => "GET",
        Method::POST => "POST",
        Method::PUT => "PUT",
        Method::DELETE => "DELETE",
        Method::PATCH => "PATCH",
        _ => "GET",
    }
}
