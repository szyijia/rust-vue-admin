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
pub struct DeleteApiReq { pub id: u64 }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApisByIdsReq { pub ids: Vec<u64> }

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

// ===== 新增接口 =====

use crate::service::system::sys_casbin;

/// GET /api/syncApi
/// 同步 API，对比已注册路由和数据库中的 API
pub async fn sync_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::fail(7005, "数据库未连接", serde_json::Value::Null));
    };
    let db = &*db;
    match sys_api::sync_api(db).await {
        Ok(result) => Json(ApiResponse::ok_with_data(serde_json::to_value(result).unwrap_or_default(), "获取成功")),
        Err(e) => { error!("同步 API 失败: {}", e); Json(ApiResponse::fail(7001, format!("同步失败: {}", e), serde_json::Value::Null)) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgnoreApiReq {
    pub path: String,
    pub method: String,
    #[serde(default)]
    pub flag: bool,
}

/// POST /api/ignoreApi
/// 忽略/取消忽略 API
pub async fn ignore_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<IgnoreApiReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    match sys_api::ignore_api(db, req.path, req.method, req.flag).await {
        Ok(_) => Json(ApiResponse::ok_msg("操作成功")),
        Err(e) => { error!("忽略 API 失败: {}", e); Json(ApiResponse::ok_msg(&format!("操作失败: {}", e))) }
    }
}

/// POST /api/enterSyncApi
/// 确认同步 API
pub async fn enter_sync_api(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<sys_api::SyncApisReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    match sys_api::enter_sync_api(db, req).await {
        Ok(_) => Json(ApiResponse::ok_msg("同步成功")),
        Err(e) => { error!("确认同步 API 失败: {}", e); Json(ApiResponse::ok_msg(&format!("同步失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiRolesReq {
    pub path: String,
    pub method: String,
}

/// GET /api/getApiRoles
/// 获取拥有指定 API 权限的角色 ID 列表
pub async fn get_api_roles(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    axum::extract::Query(req): axum::extract::Query<GetApiRolesReq>,
) -> Json<ApiResponse<Vec<u64>>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    if req.path.is_empty() || req.method.is_empty() {
        return Json(ApiResponse::err_default(7001, "API路径和请求方法不能为空"));
    }
    match sys_casbin::get_authorities_by_api(db, &req.path, &req.method).await {
        Ok(ids) => Json(ApiResponse::ok_with_data(ids, "获取成功")),
        Err(e) => { error!("获取失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetApiRolesReq {
    pub path: String,
    pub method: String,
    #[serde(default)]
    pub authority_ids: Vec<u64>,
}

/// POST /api/setApiRoles
/// 全量覆盖某 API 关联的角色列表
pub async fn set_api_roles(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetApiRolesReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    if req.path.is_empty() || req.method.is_empty() {
        return Json(ApiResponse::ok_msg("API路径和请求方法不能为空"));
    }
    match sys_casbin::set_api_authorities(db, &req.path, &req.method, req.authority_ids).await {
        Ok(_) => {
            // 刷新 Casbin 缓存
            if let Some(enforcer) = state.try_get_enforcer() {
                let _ = sys_casbin::fresh_casbin_cache(&enforcer, db).await;
            }
            Json(ApiResponse::ok_msg("设置成功"))
        }
        Err(e) => { error!("设置失败: {}", e); Json(ApiResponse::ok_msg(&format!("设置失败: {}", e))) }
    }
}

/// GET /api/freshCasbin
/// 刷新 Casbin 缓存
pub async fn fresh_casbin(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    if let Some(enforcer) = state.try_get_enforcer() {
        match sys_casbin::fresh_casbin_cache(&enforcer, db).await {
            Ok(_) => Json(ApiResponse::ok_msg("刷新成功")),
            Err(e) => { error!("刷新失败: {}", e); Json(ApiResponse::ok_msg(&format!("刷新失败: {}", e))) }
        }
    } else {
        Json(ApiResponse::ok_msg("Casbin 未初始化"))
    }
}
