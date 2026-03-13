use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_authority_btn;
use crate::utils::Claims;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthorityBtnReq {
    pub authority_id: u64,
    pub menu_id: u64,
}

/// POST /authorityBtn/getAuthorityBtn
pub async fn get_authority_btn(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetAuthorityBtnReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_authority_btn::get_authority_btn(&db, req.authority_id, req.menu_id).await {
        Ok(btn_ids) => Json(ApiResponse::ok_with_data(serde_json::json!({ "selected": btn_ids }), "获取成功")),
        Err(e) => { error!("获取按钮权限失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAuthorityBtnReq {
    pub authority_id: u64,
    pub menu_id: u64,
    #[serde(default)]
    pub selected: Vec<u64>,
}

/// POST /authorityBtn/setAuthorityBtn
pub async fn set_authority_btn(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetAuthorityBtnReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_authority_btn::set_authority_btn(&db, req.authority_id, req.menu_id, req.selected).await {
        Ok(_) => Json(ApiResponse::ok_msg("设置成功")),
        Err(e) => { error!("设置按钮权限失败: {}", e); Json(ApiResponse::ok_msg(&format!("设置失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanRemoveBtnReq {
    #[serde(default)]
    pub id: Option<String>,
}

/// POST /authorityBtn/canRemoveAuthorityBtn
/// Go 端从 c.Query("id") 获取参数
pub async fn can_remove_authority_btn(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<CanRemoveBtnReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    let authority_id: u64 = req.id.unwrap_or_default().parse().unwrap_or(0);
    match sys_authority_btn::can_remove_authority_btn(&db, authority_id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除失败: {}", e); Json(ApiResponse::ok_msg(&format!("{}", e))) }
    }
}
