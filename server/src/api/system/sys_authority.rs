use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{response::ApiResponse, AppState},
    service::system::{sys_authority, sys_casbin},
    utils::jwt::Claims,
};

// ===== 请求/响应 DTO =====

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuthorityReq {
    pub authority_id: i64,
    pub authority_name: String,
    #[serde(default)]
    pub parent_id: i64,
    #[serde(default = "default_router")]
    pub default_router: String,
}

fn default_router() -> String { "dashboard".to_string() }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAuthorityReq {
    pub authority_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthorityReq {
    pub authority_id: i64,
    pub authority_name: String,
    #[serde(default = "default_router")]
    pub default_router: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCasbinReq {
    pub authority_id: i64,
    pub casbin_infos: Vec<sys_casbin::CasbinInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyReq {
    pub authority_id: i64,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityResponse {
    pub authority: sys_authority::AuthorityInfo,
}

// ===== Handler =====

/// POST /authority/createAuthority
pub async fn create_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateAuthorityReq>,
) -> Json<ApiResponse<AuthorityResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_authority::create_authority(&db, req.authority_id, req.authority_name, req.parent_id, req.default_router).await {
        Ok(authority) => {
            if let Some(enforcer) = state.try_get_enforcer() {
                let _ = sys_casbin::fresh_casbin_cache(&enforcer, &db).await;
            }
            Json(ApiResponse::ok_with_data(AuthorityResponse { authority }, "创建成功"))
        }
        Err(e) => { error!("创建角色失败: {}", e); Json(ApiResponse::err_default(7001, &format!("创建失败: {}", e))) }
    }
}

/// POST /authority/deleteAuthority
pub async fn delete_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteAuthorityReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    match sys_authority::delete_authority(&db, req.authority_id).await {
        Ok(_) => {
            if let Some(enforcer) = state.try_get_enforcer() {
                let _ = sys_casbin::fresh_casbin_cache(&enforcer, &db).await;
            }
            Json(ApiResponse::ok_msg("删除成功"))
        }
        Err(e) => { error!("删除角色失败: {}", e); Json(ApiResponse::ok_msg(&format!("删除失败: {}", e))) }
    }
}

/// PUT /authority/updateAuthority
pub async fn update_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateAuthorityReq>,
) -> Json<ApiResponse<AuthorityResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_authority::update_authority(&db, req.authority_id, req.authority_name, req.default_router).await {
        Ok(authority) => Json(ApiResponse::ok_with_data(AuthorityResponse { authority }, "更新成功")),
        Err(e) => { error!("更新角色失败: {}", e); Json(ApiResponse::err_default(7001, &format!("更新失败: {}", e))) }
    }
}

/// POST /authority/getAuthorityList
pub async fn get_authority_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<Vec<sys_authority::AuthorityInfo>>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_authority::get_authority_list(&db).await {
        Ok(list) => Json(ApiResponse::ok_with_data(list, "获取成功")),
        Err(e) => { error!("获取角色列表失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /casbin/UpdateCasbin
pub async fn update_casbin(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateCasbinReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未初始化"));
    };
    let Some(enforcer) = state.try_get_enforcer() else {
        return Json(ApiResponse::ok_msg("Casbin 未初始化"));
    };
    match sys_casbin::update_casbin(&db, &enforcer, claims.role_id, req.authority_id, req.casbin_infos).await {
        Ok(_) => Json(ApiResponse::ok_msg("更新成功")),
        Err(e) => { error!("更新 Casbin 权限失败: {}", e); Json(ApiResponse::ok_msg(&format!("更新失败: {}", e))) }
    }
}

/// POST /casbin/getPolicyPathByAuthorityId
pub async fn get_policy_path_by_authority_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetPolicyReq>,
) -> Json<ApiResponse<Vec<sys_casbin::CasbinInfo>>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未初始化"));
    };
    match sys_casbin::get_policy_by_role(&db, req.authority_id).await {
        Ok(paths) => Json(ApiResponse::ok_with_data(paths, "获取成功")),
        Err(e) => { error!("获取权限列表失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}
