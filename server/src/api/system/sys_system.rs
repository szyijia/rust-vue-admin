// 系统配置 API - 对应 Gin-Vue-Admin server/api/v1/system/sys_system.go
use axum::extract::{Extension, State};
use axum::Json;
use serde::Serialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::initialize::get_config_path;
use crate::service::system::sys_system;
use crate::utils::Claims;

/// 获取系统配置响应结构（对应 Go SysConfigResponse）
#[derive(Debug, Serialize)]
pub struct SysConfigResponse {
    pub config: serde_json::Value,
}

/// snake_case -> kebab-case 特殊映射表
/// Rust config.yaml 中某些 key 的命名风格与 gin-vue-admin 不一致，需要做特殊处理
const SNAKE_TO_KEBAB_SPECIAL: &[(&str, &str)] = &[
    // 顶层 key：Rust 用 log，gin-vue-admin 用 zap
    ("log", "zap"),
    // system 子字段：Rust 用 ip_limit_*，gin-vue-admin 用 iplimit-*（无中间横杠）
    ("ip_limit_count", "iplimit-count"),
    ("ip_limit_time", "iplimit-time"),
    // redis 子字段：Go 版本使用 camelCase 而非 kebab-case
    ("use_cluster", "useCluster"),
    ("cluster_addrs", "clusterAddrs"),
];

/// 将 JSON Value 中所有 object key 从 snake_case 转换为 kebab-case
/// 同时处理特殊 key 映射（如 log -> zap, ip_limit_count -> iplimit-count）
fn snake_to_kebab_keys(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                // 先查特殊映射表，没有则通用 _ -> -
                let new_key = SNAKE_TO_KEBAB_SPECIAL
                    .iter()
                    .find(|(from, _)| *from == k.as_str())
                    .map(|(_, to)| to.to_string())
                    .unwrap_or_else(|| k.replace('_', "-"));
                new_map.insert(new_key, snake_to_kebab_keys(v));
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(snake_to_kebab_keys).collect())
        }
        other => other.clone(),
    }
}

/// kebab-case -> snake_case 特殊映射表（与 SNAKE_TO_KEBAB_SPECIAL 反向对应）
const KEBAB_TO_SNAKE_SPECIAL: &[(&str, &str)] = &[
    ("zap", "log"),
    ("iplimit-count", "ip_limit_count"),
    ("iplimit-time", "ip_limit_time"),
    // redis 子字段：Go 版本使用 camelCase
    ("useCluster", "use_cluster"),
    ("clusterAddrs", "cluster_addrs"),
];

/// 将 JSON Value 中所有 object key 从 kebab-case 转换为 snake_case
/// 同时处理特殊 key 映射（如 zap -> log, iplimit-count -> ip_limit_count）
fn kebab_to_snake_keys(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                // 先查特殊映射表，没有则通用 - -> _
                let new_key = KEBAB_TO_SNAKE_SPECIAL
                    .iter()
                    .find(|(from, _)| *from == k.as_str())
                    .map(|(_, to)| to.to_string())
                    .unwrap_or_else(|| k.replace('-', "_"));
                new_map.insert(new_key, kebab_to_snake_keys(v));
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(kebab_to_snake_keys).collect())
        }
        other => other.clone(),
    }
}

/// POST /system/getSystemConfig
/// 获取配置文件内容（对应 Go GetSystemConfig）
/// 注意：Go 版本返回的配置字段使用 kebab-case（如 db-type, oss-type），因为对应 YAML 配置文件格式
pub async fn get_system_config(
    State(_state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<serde_json::Value>> {
    // 读取配置文件内容作为 YAML 值返回（转换为 kebab-case 格式与 Go 版本一致）
    let config_path = get_config_path();
    match std::fs::read_to_string(&config_path) {
        Ok(content) => {
            match serde_yaml::from_str::<serde_json::Value>(&content) {
                Ok(config_value) => {
                    // 将 snake_case key 转换为 kebab-case（与 gin-vue-admin 前端一致）
                    let mut kebab_config = snake_to_kebab_keys(&config_value);

                    // 如果配置中没有 autocode 字段，注入占位默认值
                    // Rust 版本暂不支持 autocode 功能，但前端页面需要该字段才能正常渲染
                    if let serde_json::Value::Object(ref mut map) = kebab_config {
                        if !map.contains_key("autocode") {
                            map.insert("autocode".to_string(), serde_json::json!({
                                "transfer-restart": false,
                                "root": "",
                                "server": "",
                                "server-api": "",
                                "server-initialize": "",
                                "server-model": "",
                                "server-request": "",
                                "server-router": "",
                                "server-service": "",
                                "web": "",
                                "web-api": "",
                                "web-form": "",
                                "web-table": "",
                                "module": "",
                                "ai-path": ""
                            }));
                        }
                    }

                    Json(ApiResponse::ok_with_data(
                        serde_json::json!({ "config": kebab_config }),
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

    // 将前端传来的 kebab-case key 转换为 snake_case（与 Rust config.yaml 一致）
    let snake_config = kebab_to_snake_keys(&config_value);

    // 将配置转为 YAML 写入文件（写入当前优先的配置文件）
    let config_path = get_config_path();
    match serde_yaml::to_string(&snake_config) {
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
