use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_operation_record;
use crate::utils::Claims;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordListReq {
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

/// GET /sysOperationRecord/getSysOperationRecordList
/// Go 端使用 c.ShouldBindQuery，即 GET + query
pub async fn get_sys_operation_record_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetRecordListReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_operation_record::get_operation_record_list(&db, req.method, req.path, req.status, req.page, req.page_size).await {
        Ok(result) => Json(ApiResponse::ok_with_data(serde_json::to_value(result).unwrap_or_default(), "获取成功")),
        Err(e) => { error!("获取操作记录列表失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRecordReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// DELETE /sysOperationRecord/deleteSysOperationRecord
pub async fn delete_sys_operation_record(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteRecordReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_operation_record::delete_operation_record(&db, req.id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除操作记录失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRecordsByIdsReq {
    pub ids: Vec<u64>,
}

/// DELETE /sysOperationRecord/deleteSysOperationRecordByIds
pub async fn delete_sys_operation_record_by_ids(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteRecordsByIdsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_operation_record::delete_operation_records_by_ids(&db, req.ids).await {
        Ok(_) => Json(ApiResponse::ok_msg("批量删除成功")),
        Err(e) => { error!("批量删除操作记录失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindRecordReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// GET /sysOperationRecord/findSysOperationRecord
pub async fn find_sys_operation_record(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<FindRecordReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_operation_record::find_operation_record(&db, req.id).await {
        Ok(record) => Json(ApiResponse::ok_with_data(serde_json::json!({ "reSysOperationRecord": record }), "查询成功")),
        Err(e) => { error!("查询操作记录失败: {}", e); Json(ApiResponse::fail(7001, format!("查询失败: {}", e), serde_json::Value::Null)) }
    }
}
