use redis::{aio::ConnectionManager, Client};
use tracing::info;

use crate::config::RedisConfig;

/// 初始化 Redis 连接，对应 Gin-Vue-Admin 的 initialize.Redis()
///
/// 使用 ConnectionManager 自动重连，无需手动管理连接生命周期
/// 支持单机模式（use_cluster=false）
pub async fn init_redis(cfg: &RedisConfig) -> anyhow::Result<ConnectionManager> {
    info!(addr = %cfg.addr, "正在连接 Redis...");

    // 构建连接 URL
    let url = build_redis_url(cfg);

    let client = Client::open(url.as_str())
        .map_err(|e| anyhow::anyhow!("Redis 客户端创建失败: {}", e))?;

    // ConnectionManager 会自动处理断线重连
    let manager = ConnectionManager::new(client)
        .await
        .map_err(|e| anyhow::anyhow!("Redis 连接失败: {}，请检查 Redis 服务是否启动", e))?;

    info!(addr = %cfg.addr, "✅ Redis 连接成功");

    Ok(manager)
}

/// 构建 Redis 连接 URL
/// 格式：redis://:password@host:port/db
fn build_redis_url(cfg: &RedisConfig) -> String {
    if cfg.password.is_empty() {
        format!("redis://{}/{}", cfg.addr, cfg.db)
    } else {
        format!("redis://:{}@{}/{}", cfg.password, cfg.addr, cfg.db)
    }
}

/// 测试 Redis 连接是否正常（PING）
#[allow(dead_code)]
pub async fn ping_redis(conn: &mut ConnectionManager) -> bool {
    let result: redis::RedisResult<String> = redis::cmd("PING").query_async(conn).await;
    matches!(result, Ok(ref s) if s == "PONG")
}
