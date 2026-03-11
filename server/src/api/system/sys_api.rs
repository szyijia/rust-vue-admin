use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{response::ApiResponse, AppState},
    service::system::sys_api::{self, ApiInfo},
    utils::jwt::Claims,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiReq {
    pub path: String,
    pub description: String,
    pub api_group: String,
    pub method: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApiReq { pub id: i32 }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApisByIdsReq { pub ids: Vec<i32> }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiReq {
    pub id: i32,
    pub path: String,
    pub description: String,
    pub api_group: String,
    pub method: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiListReq {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    pub path: Option<String>,
    pub api_group: Option<String>,
    pub method: Option<String>,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApiListResponse {
    pub list: Vec<ApiInfo>,
    pub total: u64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllApisResponse { pub apis: Vec<ApiInfo> }

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApiGroupsResponse {
    pub groups: Vec<String>,
}

/// POST /api/createApi
pub async fn create_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateApiReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::create_api(db, req.path, req.description, req.api_group, req.method).await {
        Ok(_) => Json(ApiResponse::ok_msg("创建成功")),
        Err(e) => { error!("创建 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("创建失败: {}", e))) }
    }
}

/// POST /api/deleteApi
pub async fn delete_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteApiReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::delete_api(db, req.id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("删除失败: {}", e))) }
    }
}

/// DELETE /api/deleteApisByIds
pub async fn delete_apis_by_ids(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteApisByIdsReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::delete_apis_by_ids(db, req.ids).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("批量删除 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("删除失败: {}", e))) }
    }
}

/// POST /api/updateApi
pub async fn update_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateApiReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::update_api(db, req.id, req.path, req.description, req.api_group, req.method).await {
        Ok(_) => Json(ApiResponse::ok_msg("修改成功")),
        Err(e) => { error!("更新 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("修改失败: {}", e))) }
    }
}

/// POST /api/getApiList
pub async fn get_api_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetApiListReq>,
) -> Json<ApiResponse<ApiListResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::get_api_list(db, req.page, req.page_size, req.path, req.api_group, req.method).await {
        Ok((list, total)) => Json(ApiResponse::ok_with_data(
            ApiListResponse { list, total, page: req.page, page_size: req.page_size },
            "获取成功",
        )),
        Err(e) => { error!("获取 API 列表失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// GET /api/getApiGroups
/// 获取所有 API 分组列表
pub async fn get_api_groups(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<ApiGroupsResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::get_all_apis(db).await {
        Ok(apis) => {
            let mut groups: Vec<String> = apis.into_iter()
                .map(|a| a.api_group)
                .filter(|g| !g.is_empty())
                .collect();
            groups.sort();
            groups.dedup();
            Json(ApiResponse::ok_with_data(ApiGroupsResponse { groups }, "获取成功"))
        }
        Err(e) => { error!("获取 API 分组失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /api/getAllApis
pub async fn get_all_apis(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<AllApisResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::get_all_apis(db).await {
        Ok(apis) => Json(ApiResponse::ok_with_data(AllApisResponse { apis }, "获取成功")),
        Err(e) => { error!("获取所有 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /api/getApiById
/// 根据 ID 获取单个 API，对应 Gin-Vue-Admin 的 apiRouterGroup.POST("getApiById")
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiByIdReq {
    #[serde(alias = "id", alias = "ID")]
    pub id: i32,
}

pub async fn get_api_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetApiByIdReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_api::get_api_by_id(db, req.id).await {
        Ok(Some(api)) => Json(ApiResponse::ok_with_data(serde_json::json!({ "api": api }), "获取成功")),
        Ok(None) => Json(ApiResponse::err_default(7001, "API 不存在")),
        Err(e) => { error!("获取 API 失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}
