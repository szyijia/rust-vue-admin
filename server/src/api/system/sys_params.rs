use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_params;
use crate::utils::Claims;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParamsReq {
    pub name: String,
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub desc: String,
}

/// POST /sysParams/createSysParams
pub async fn create_sys_params(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateParamsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_params::create_params(&db, req.name, req.key, req.value, req.desc).await {
        Ok(_) => Json(ApiResponse::ok_msg("创建成功")),
        Err(e) => { error!("创建参数失败: {}", e); Json(ApiResponse::ok_msg(&format!("创建失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteParamsReq {
    #[serde(rename = "ID")]
    pub id: String,
}

/// DELETE /sysParams/deleteSysParams
/// Go 端从 c.Query("ID") 获取参数
pub async fn delete_sys_params(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<DeleteParamsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    let id: i64 = req.id.parse().unwrap_or(0);
    match sys_params::delete_params(&db, id as u64).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除参数失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteParamsByIdsReq {
    /// Go 端从 c.QueryArray("IDs[]") 获取，前端用 params 传
    #[serde(rename = "IDs[]", default)]
    pub ids: Vec<String>,
}

/// DELETE /sysParams/deleteSysParamsByIds
/// Go 端从 c.QueryArray("IDs[]") 获取参数
pub async fn delete_sys_params_by_ids(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<DeleteParamsByIdsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    let ids: Vec<u64> = req.ids.iter().filter_map(|s| s.parse().ok()).collect();
    match sys_params::delete_params_by_ids(&db, ids).await {
        Ok(_) => Json(ApiResponse::ok_msg("批量删除成功")),
        Err(e) => { error!("批量删除参数失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParamsReq {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub desc: String,
}

/// PUT /sysParams/updateSysParams
pub async fn update_sys_params(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateParamsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_params::update_params(&db, req.id, req.name, req.key, req.value, req.desc).await {
        Ok(_) => Json(ApiResponse::ok_msg("更新成功")),
        Err(e) => { error!("更新参数失败: {}", e); Json(ApiResponse::ok_msg(&format!("更新失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindParamsReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// GET /sysParams/findSysParams
pub async fn find_sys_params(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<FindParamsReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_params::find_params(&db, req.id).await {
        Ok(param) => Json(ApiResponse::ok_with_data(serde_json::to_value(param).unwrap_or_default(), "查询成功")),
        Err(e) => { error!("查询参数失败: {}", e); Json(ApiResponse::fail(7001, format!("查询失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParamsListReq {
    pub name: Option<String>,
    pub key: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

/// GET /sysParams/getSysParamsList
/// Go 端使用 c.ShouldBindQuery，即 GET + query
pub async fn get_sys_params_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetParamsListReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_params::get_params_list(&db, req.name, req.key, req.page, req.page_size).await {
        Ok(result) => Json(ApiResponse::ok_with_data(serde_json::to_value(result).unwrap_or_default(), "获取成功")),
        Err(e) => { error!("获取参数列表失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParamsByKeyReq {
    pub key: String,
}

/// GET /sysParams/getSysParamsByKey
pub async fn get_sys_params_by_key(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetParamsByKeyReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_params::get_params_by_key(&db, &req.key).await {
        Ok(param) => Json(ApiResponse::ok_with_data(serde_json::to_value(param).unwrap_or_default(), "查询成功")),
        Err(e) => { error!("查询参数失败: {}", e); Json(ApiResponse::fail(7001, format!("查询失败: {}", e), serde_json::Value::Null)) }
    }
}
