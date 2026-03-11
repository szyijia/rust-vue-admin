use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;
use tracing::warn;

use crate::global::{ApiResponse, AppState};

/// IP 限流中间件，对应 Gin-Vue-Admin 的 middleware.LimitCountIP()
///
/// 基于 Redis 滑动窗口算法实现 IP 级别的请求限流
/// 配置项：system.ip_limit_count（次数）/ system.ip_limit_time（时间窗口秒数）
pub async fn ip_rate_limit(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, axum::response::Response> {
    // 未启用 Redis 时跳过限流
    if !state.config.system.use_redis {
        return Ok(next.run(req).await);
    }

    let redis_conn = match &state.redis {
        Some(conn) => conn.clone(),
        None => return Ok(next.run(req).await),
    };

    let ip = addr.ip().to_string();
    let limit_count = state.config.system.ip_limit_count;
    let limit_time = state.config.system.ip_limit_time;

    // 执行限流检查
    match check_ip_limit(&ip, limit_count, limit_time, redis_conn.as_ref().clone()).await {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => {
            warn!(ip = %ip, "IP 请求超出限制");
            let resp: ApiResponse<()> = ApiResponse::too_many_requests("请求过于频繁，请稍后再试");
            Err(resp.into_http_response())
        }
        Err(e) => {
            // Redis 操作失败时放行（降级处理，不影响正常业务）
            warn!(ip = %ip, error = %e, "限流检查失败，已降级放行");
            Ok(next.run(req).await)
        }
    }
}

/// 使用 Redis INCR + EXPIRE 实现滑动窗口限流
///
/// 返回 true 表示允许通过，false 表示超出限制
async fn check_ip_limit(
    ip: &str,
    limit_count: u32,
    limit_time: u32,
    mut conn: redis::aio::ConnectionManager,
) -> anyhow::Result<bool> {
    let key = format!("ip_limit:{}", ip);

    // INCR 原子递增，首次创建时返回 1
    let count: u64 = redis::cmd("INCR").arg(&key).query_async(&mut conn).await?;

    // 首次访问时设置过期时间
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(&key)
            .arg(limit_time)
            .query_async::<()>(&mut conn)
            .await?;
    }

    Ok(count <= limit_count as u64)
}
