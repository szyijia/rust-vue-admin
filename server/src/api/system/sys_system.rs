// 系统配置 API - 对应 Gin-Vue-Admin server/api/v1/system/sys_system.go
use axum::extract::{Extension, State};
use axum::Json;
use serde::Serialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_system;
use crate::utils::Claims;

/// 获取系统配置响应结构（对应 Go SysConfigResponse）
#[derive(Debug, Serialize)]
pub struct SysConfigResponse {
    pub config: serde_json::Value,
}

/// POST /system/getSystemConfig
/// 获取配置文件内容（对应 Go GetSystemConfig）
/// 注意：Go 版本返回的配置字段使用 kebab-case（如 db-type, oss-type），因为对应 YAML 配置文件格式
pub async fn get_system_config(
    State(_state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<serde_json::Value>> {
    // 读取配置文件内容作为 YAML 值返回（保持 kebab-case 格式与 Go 一致）
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    match std::fs::read_to_string(&config_path) {
        Ok(content) => {
            match serde_yaml::from_str::<serde_json::Value>(&content) {
                Ok(config_value) => {
                    Json(ApiResponse::ok_with_data(
                        serde_json::json!({ "config": config_value }),
                        "获取成功",
                    ))
                }
                Err(e) => {
                    error!("解析配置文件失败: {}", e);
                    Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null))
                }
            }
        }
        Err(e) => {
            error!("读取配置文件失败: {}", e);
            Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null))
        }
    }
}

/// POST /system/setSystemConfig
/// 设置配置文件内容（对应 Go SetSystemConfig）
pub async fn set_system_config(
    State(_state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(body): Json<serde_json::Value>,
) -> Json<ApiResponse<()>> {
    // 从请求体中取出 config 字段
    let config_value = if let Some(config) = body.get("config") {
        config.clone()
    } else {
        body.clone()
    };

    // 将配置转为 YAML 写入文件
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    match serde_yaml::to_string(&config_value) {
        Ok(yaml_content) => {
            match std::fs::write(&config_path, &yaml_content) {
                Ok(_) => {
                    // 配置文件写入后，config_watcher 会自动检测变更并热重载
                    Json(ApiResponse::ok_msg("设置成功"))
                }
                Err(e) => {
                    error!("写入配置文件失败: {}", e);
                    Json(ApiResponse::fail(7001, format!("设置失败: {}", e), ()))
                }
            }
        }
        Err(e) => {
            error!("序列化配置失败: {}", e);
            Json(ApiResponse::fail(7001, format!("设置失败: {}", e), ()))
        }
    }
}

/// POST /system/getServerInfo
/// 获取服务器信息（对应 Go GetServerInfo）
pub async fn get_server_info(
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<serde_json::Value>> {
    match sys_system::get_server_info() {
        Ok(server) => {
            Json(ApiResponse::ok_with_data(
                serde_json::json!({ "server": server }),
                "获取成功",
            ))
        }
        Err(e) => {
            error!("获取服务器信息失败: {}", e);
            Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null))
        }
    }
}

/// POST /system/reloadSystem
/// 重载系统（对应 Go ReloadSystem）
/// 在 Rust 版本中，触发配置热重载
pub async fn reload_system(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<()>> {
    // 重新读取配置文件并更新到 AppState
    match crate::initialize::config::load_config() {
        Ok(new_config) => {
            state.set_config(new_config).await;
            Json(ApiResponse::ok_msg("重载系统成功"))
        }
        Err(e) => {
            error!("重载系统失败: {}", e);
            Json(ApiResponse::fail(7001, format!("重载系统失败: {}", e), ()))
        }
    }
}
