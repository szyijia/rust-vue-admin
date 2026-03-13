use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_error;

/// 创建错误日志请求体，对应 gin-vue-admin 的 SysError
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSysErrorReq {
    /// 错误来源
    pub form: Option<String>,
    /// 错误内容
    pub info: Option<String>,
    /// 日志等级
    pub level: Option<String>,
}

/// POST /sysError/createSysError
/// 创建错误日志（无需认证，前端 error-handler 自动调用）
pub async fn create_sys_error(
    State(state): State<AppState>,
    Json(req): Json<CreateSysErrorReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_error::create_sys_error(&db, req.form, req.info, req.level).await {
        Ok(_) => Json(ApiResponse::ok_msg("创建成功")),
        Err(e) => {
            error!("创建错误日志失败: {}", e);
            Json(ApiResponse::fail_msg(7001, format!("创建失败:{}", e)))
        }
    }
}
