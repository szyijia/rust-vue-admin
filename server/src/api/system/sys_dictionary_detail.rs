use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_dictionary_detail;
use crate::utils::Claims;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDetailReq {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub extend: String,
    #[serde(default = "default_true")]
    pub status: bool,
    #[serde(default)]
    pub sort: i64,
    pub sys_dictionary_id: u64,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u64>,
}

fn default_true() -> bool { true }

/// POST /sysDictionaryDetail/createSysDictionaryDetail
pub async fn create_sys_dictionary_detail(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateDetailReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary_detail::create_dictionary_detail(
        &db, req.label, req.value, req.extend, req.status, req.sort, req.sys_dictionary_id, req.parent_id,
    ).await {
        Ok(_) => Json(ApiResponse::ok_msg("创建成功")),
        Err(e) => { error!("创建字典详情失败: {}", e); Json(ApiResponse::fail(7001, format!("创建失败: {}", e), ())) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDetailReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// DELETE /sysDictionaryDetail/deleteSysDictionaryDetail
pub async fn delete_sys_dictionary_detail(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteDetailReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary_detail::delete_dictionary_detail(&db, req.id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除字典详情失败: {}", e); Json(ApiResponse::fail(7001, format!("删除失败: {}", e), ())) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDetailReq {
    #[serde(rename = "ID")]
    pub id: u64,
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub extend: String,
    #[serde(default = "default_true")]
    pub status: bool,
    #[serde(default)]
    pub sort: i64,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u64>,
}

/// PUT /sysDictionaryDetail/updateSysDictionaryDetail
pub async fn update_sys_dictionary_detail(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateDetailReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary_detail::update_dictionary_detail(
        &db, req.id, req.label, req.value, req.extend, req.status, req.sort, req.parent_id,
    ).await {
        Ok(_) => Json(ApiResponse::ok_msg("更新成功")),
        Err(e) => { error!("更新字典详情失败: {}", e); Json(ApiResponse::fail(7001, format!("更新失败: {}", e), ())) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindDetailReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// GET /sysDictionaryDetail/findSysDictionaryDetail
pub async fn find_sys_dictionary_detail(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<FindDetailReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_dictionary_detail::find_dictionary_detail(&db, req.id).await {
        Ok(detail) => Json(ApiResponse::ok_with_data(serde_json::json!({ "reSysDictionaryDetail": detail }), "查询成功")),
        Err(e) => { error!("查询字典详情失败: {}", e); Json(ApiResponse::fail(7001, format!("查询失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDetailListReq {
    #[serde(alias = "sysDictionaryID")]
    pub sys_dictionary_id: u64,
    pub label: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

/// GET /sysDictionaryDetail/getSysDictionaryDetailList
pub async fn get_sys_dictionary_detail_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetDetailListReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_dictionary_detail::get_dictionary_detail_list(&db, req.sys_dictionary_id, req.label, req.page, req.page_size).await {
        Ok(result) => Json(ApiResponse::ok_with_data(serde_json::to_value(result).unwrap_or_default(), "获取成功")),
        Err(e) => { error!("获取字典详情列表失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

// ===== 以下为新增的4个字典树形接口 =====

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTreeListReq {
    #[serde(alias = "sysDictionaryID")]
    pub sys_dictionary_id: u64,
}

/// GET /sysDictionaryDetail/getDictionaryTreeList
/// 获取字典详情树形结构（对应 Go GetDictionaryTreeList）
pub async fn get_dictionary_tree_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetTreeListReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_dictionary_detail::get_dictionary_tree_list(&db, req.sys_dictionary_id).await {
        Ok(list) => Json(ApiResponse::ok_with_data(serde_json::json!({ "list": list }), "获取成功")),
        Err(e) => { error!("获取字典树形列表失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTreeListByTypeReq {
    #[serde(rename = "type")]
    pub dict_type: String,
}

/// GET /sysDictionaryDetail/getDictionaryTreeListByType
/// 根据字典类型获取树形结构（对应 Go GetDictionaryTreeListByType）
pub async fn get_dictionary_tree_list_by_type(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetTreeListByTypeReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    if req.dict_type.is_empty() {
        return Json(ApiResponse::fail(7001, "字典类型不能为空", serde_json::Value::Null));
    }
    match sys_dictionary_detail::get_dictionary_tree_list_by_type(&db, &req.dict_type).await {
        Ok(list) => Json(ApiResponse::ok_with_data(serde_json::json!({ "list": list }), "获取成功")),
        Err(e) => { error!("获取字典树形列表失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDetailsByParentReq {
    #[serde(alias = "sysDictionaryID")]
    pub sys_dictionary_id: u64,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u64>,
    #[serde(default)]
    pub include_children: bool,
}

/// GET /sysDictionaryDetail/getDictionaryDetailsByParent
/// 根据父级ID获取字典详情（对应 Go GetDictionaryDetailsByParent）
pub async fn get_dictionary_details_by_parent(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetDetailsByParentReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_dictionary_detail::get_dictionary_details_by_parent(&db, req.sys_dictionary_id, req.parent_id, req.include_children).await {
        Ok(list) => Json(ApiResponse::ok_with_data(serde_json::json!({ "list": list }), "获取成功")),
        Err(e) => { error!("获取字典详情失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDictionaryPathReq {
    pub id: u64,
}

/// GET /sysDictionaryDetail/getDictionaryPath
/// 获取字典详情的完整路径（对应 Go GetDictionaryPath）
pub async fn get_dictionary_path(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetDictionaryPathReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    if req.id == 0 {
        return Json(ApiResponse::fail(7001, "字典详情ID不能为空", serde_json::Value::Null));
    }
    match sys_dictionary_detail::get_dictionary_path(&db, req.id).await {
        Ok(path) => Json(ApiResponse::ok_with_data(serde_json::json!({ "path": path }), "获取成功")),
        Err(e) => { error!("获取字典路径失败: {}", e); Json(ApiResponse::fail(7001, format!("获取失败: {}", e), serde_json::Value::Null)) }
    }
}
