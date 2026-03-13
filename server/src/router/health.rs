use axum::extract::State;
use chrono::Local;
use serde::Serialize;

use crate::global::{ApiResponse, AppState};

/// 健康检查响应数据
#[derive(Debug, Serialize)]
pub struct HealthData {
    /// 服务状态
    pub status: String,
    /// 服务版本
    pub version: String,
    /// 当前服务器时间
    pub timestamp: String,
    /// 运行环境
    pub env: String,
    /// 数据库连接状态
    pub db: ComponentStatus,
    /// Redis 连接状态
    pub redis: ComponentStatus,
}

/// 组件连接状态
#[derive(Debug, Serialize)]
pub struct ComponentStatus {
    /// 是否已启用
    pub enabled: bool,
    /// 连接是否正常
    pub connected: bool,
    /// 状态描述
    pub message: String,
}

/// GET /health - 健康检查接口
///
/// 返回服务运行状态及各组件连接情况，无需认证
pub async fn health_check(State(state): State<AppState>) -> ApiResponse<HealthData> {
    // 获取当前配置快照（支持热重载）
    let config = state.get_config();

    // 检查数据库状态
    let db_status = check_db_status(&state, &config).await;

    // 检查 Redis 状态
    let redis_status = check_redis_status(&state, &config).await;

    // 整体状态：所有已启用的组件都正常才算 ok
    let overall_ok = (!db_status.enabled || db_status.connected)
        && (!redis_status.enabled || redis_status.connected);

    ApiResponse::ok(HealthData {
        status: if overall_ok { "ok" } else { "degraded" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        env: config.system.env.clone(),
        db: db_status,
        redis: redis_status,
    })
}

/// 检查数据库连接状态
async fn check_db_status(state: &AppState, config: &crate::config::AppConfig) -> ComponentStatus {
    match state.try_get_db() {
        None => ComponentStatus {
            enabled: false,
            connected: false,
            message: "未初始化（请访问 /init/initdb 初始化数据库）".to_string(),
        },
        Some(db) => match db.ping().await {
            Ok(_) => ComponentStatus {
                enabled: true,
                connected: true,
                message: format!("已连接 ({})", config.system.db_type),
            },
            Err(e) => ComponentStatus {
                enabled: true,
                connected: false,
                message: format!("连接异常: {}", e),
            },
        },
    }
}

/// 检查 Redis 连接状态
async fn check_redis_status(state: &AppState, config: &crate::config::AppConfig) -> ComponentStatus {
    if !config.system.use_redis {
        return ComponentStatus {
            enabled: false,
            connected: false,
            message: "未启用 (use_redis=false)".to_string(),
        };
    }

    match &state.redis {
        None => ComponentStatus {
            enabled: true,
            connected: false,
            message: "连接未建立".to_string(),
        },
        Some(conn) => {
            // 克隆 ConnectionManager 执行 PING（ConnectionManager 内部是 Arc，clone 很廉价）
            let mut conn = conn.as_ref().clone();
            let ping_result: redis::RedisResult<String> =
                redis::cmd("PING").query_async(&mut conn).await;
            match ping_result {
                Ok(_) => ComponentStatus {
                    enabled: true,
                    connected: true,
                    message: format!("已连接 ({})", config.redis.addr),
                },
                Err(e) => ComponentStatus {
                    enabled: true,
                    connected: false,
                    message: format!("连接异常: {}", e),
                },
            }
        }
    }
}
