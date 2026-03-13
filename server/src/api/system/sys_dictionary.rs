use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::Deserialize;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::service::system::sys_dictionary;
use crate::utils::Claims;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDictionaryReq {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default = "default_true")]
    pub status: bool,
    #[serde(default)]
    pub desc: String,
    pub parent_id: Option<i64>,
}

fn default_true() -> bool { true }

/// POST /sysDictionary/createSysDictionary
pub async fn create_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateDictionaryReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary::create_dictionary(&db, req.name, req.r#type, req.status, req.desc).await {
        Ok(_) => Json(ApiResponse::ok_msg("创建成功")),
        Err(e) => { error!("创建字典失败: {}", e); Json(ApiResponse::ok_msg(&format!("创建失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDictionaryReq {
    #[serde(rename = "ID")]
    pub id: u64,
}

/// DELETE /sysDictionary/deleteSysDictionary
pub async fn delete_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteDictionaryReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary::delete_dictionary(&db, req.id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除字典失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDictionaryReq {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default = "default_true")]
    pub status: bool,
    #[serde(default)]
    pub desc: String,
    pub parent_id: Option<i64>,
}

/// PUT /sysDictionary/updateSysDictionary
pub async fn update_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateDictionaryReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary::update_dictionary(&db, req.id, req.name, req.r#type, req.status, req.desc).await {
        Ok(_) => Json(ApiResponse::ok_msg("更新成功")),
        Err(e) => { error!("更新字典失败: {}", e); Json(ApiResponse::ok_msg(&format!("更新失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindDictionaryReq {
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
    #[serde(rename = "ID", default)]
    pub id: Option<u64>,
    pub status: Option<bool>,
}

/// GET /sysDictionary/findSysDictionary
pub async fn find_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<FindDictionaryReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null));
    };
    match sys_dictionary::find_dictionary(&db, req.r#type, req.id, req.status).await {
        Ok(info) => Json(ApiResponse::ok_with_data(
            serde_json::json!({ "resysDictionary": info }),
            "查询成功",
        )),
        Err(e) => { error!("查询字典失败: {}", e); Json(ApiResponse::fail(7001, format!("字典未创建或未开启"), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDictionaryListReq {
    pub name: Option<String>,
}

/// GET /sysDictionary/getSysDictionaryList
pub async fn get_sys_dictionary_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<GetDictionaryListReq>,
) -> Json<ApiResponse<Vec<sys_dictionary::DictionaryInfo>>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_dictionary::get_dictionary_list(&db, req.name).await {
        Ok(list) => Json(ApiResponse::ok_with_data(list, "获取成功")),
        Err(e) => { error!("获取字典列表失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// GET /sysDictionary/exportSysDictionary
pub async fn export_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(req): Query<DeleteDictionaryReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_dictionary::export_dictionary(&db, req.id).await {
        Ok(data) => Json(ApiResponse::ok_with_data(data, "导出成功")),
        Err(e) => { error!("导出字典失败: {}", e); Json(ApiResponse::err_default(7001, &format!("导出失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportDictionaryReq {
    pub json: String,
}

/// POST /sysDictionary/importSysDictionary
pub async fn import_sys_dictionary(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<ImportDictionaryReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_dictionary::import_dictionary(&db, &req.json).await {
        Ok(_) => Json(ApiResponse::ok_msg("导入成功")),
        Err(e) => { error!("导入字典失败: {}", e); Json(ApiResponse::ok_msg(&format!("导入失败: {}", e))) }
    }
}
