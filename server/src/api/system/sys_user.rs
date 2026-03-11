use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{response::ApiResponse, AppState},
    service::UserService,
    utils::jwt::Claims,
};

// ===== 请求/响应 DTO =====

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserListReq {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserReq {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAuthorityReq {
    pub id: i64,
    pub authority_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUserInfoReq {
    pub id: i64,
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub header_img: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordReq {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUserEnableReq {
    pub id: i64,
    /// 0=禁用，1=启用
    pub enable: i8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserItem {
    /// 兼容前端 Gin-Vue-Admin 使用大写 ID 的惯例
    #[serde(rename = "ID")]
    pub id: i64,
    pub uuid: String,
    pub user_name: String,
    pub nick_name: String,
    pub header_img: String,
    pub phone: String,
    pub email: String,
    pub enable: i8,
    pub authority_id: i64,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    pub list: Vec<UserItem>,
    pub total: u64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponse {
    pub user_info: Option<UserItem>,
}

// ===== Handler =====

/// POST /user/getUserList
/// 获取用户列表（分页），对应 Gin-Vue-Admin 的 userApi.GetUserList()
pub async fn get_user_list(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetUserListReq>,
) -> Json<ApiResponse<UserListResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match UserService::get_user_list(db, req.page, req.page_size).await {
        Ok(result) => {
            let list = result.list.into_iter().map(|u| UserItem {
                id: u.id,
                uuid: u.uuid.to_string(),
                user_name: u.username,
                nick_name: u.nick_name,
                header_img: u.header_img,
                phone: u.phone,
                email: u.email,
                enable: u.enable,
                authority_id: u.authority_id,
            }).collect();
            Json(ApiResponse::ok_with_data(UserListResponse {
                list,
                total: result.total,
                page: result.page,
                page_size: result.page_size,
            }, "获取成功"))
        }
        Err(e) => {
            error!("获取用户列表失败: {}", e);
            Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e)))
        }
    }
}

/// DELETE /user/deleteUser
/// 删除用户（软删除），对应 Gin-Vue-Admin 的 userApi.DeleteUser()
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<DeleteUserReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    match UserService::delete_user(db, req.id, claims.user_id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => {
            error!("删除用户失败: {}", e);
            Json(ApiResponse::ok_msg(&format!("删除失败: {}", e)))
        }
    }
}

/// POST /user/setUserAuthority
/// 设置用户角色，对应 Gin-Vue-Admin 的 userApi.SetUserAuthority()
pub async fn set_user_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetUserAuthorityReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    match UserService::set_user_authority(db, req.id, req.authority_id).await {
        Ok(_) => Json(ApiResponse::ok_msg("设置成功")),
        Err(e) => {
            error!("设置用户角色失败: {}", e);
            Json(ApiResponse::ok_msg(&format!("设置失败: {}", e)))
        }
    }
}

/// PUT /user/setUserInfo
/// 修改用户信息（管理员操作），对应 Gin-Vue-Admin 的 userApi.SetUserInfo()
pub async fn set_user_info(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetUserInfoReq>,
) -> Json<ApiResponse<UserInfoResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match UserService::update_user_info(db, req.id, req.nick_name, req.phone, req.email, req.header_img).await {
        Ok(user) => {
            let item = UserItem {
                id: user.id,
                uuid: user.uuid.to_string(),
                user_name: user.username,
                nick_name: user.nick_name,
                header_img: user.header_img,
                phone: user.phone,
                email: user.email,
                enable: user.enable,
                authority_id: user.authority_id,
            };
            Json(ApiResponse::ok_with_data(UserInfoResponse { user_info: Some(item) }, "修改成功"))
        }
        Err(e) => {
            error!("修改用户信息失败: {}", e);
            Json(ApiResponse::err_default(7001, &format!("修改失败: {}", e)))
        }
    }
}

/// POST /user/resetPassword
/// 重置用户密码（管理员操作），对应 Gin-Vue-Admin 的 userApi.ResetPassword()
pub async fn reset_password(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<ResetPasswordReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    // 重置为默认密码 "a123456"
    match UserService::reset_password(db, req.id, "a123456").await {
        Ok(_) => Json(ApiResponse::ok_msg("密码已重置为 a123456")),
        Err(e) => {
            error!("重置密码失败: {}", e);
            Json(ApiResponse::ok_msg(&format!("重置失败: {}", e)))
        }
    }
}

/// POST /user/setUserEnable
/// 设置用户启用/禁用状态
pub async fn set_user_enable(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetUserEnableReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    match UserService::set_user_enable(db, req.id, req.enable).await {
        Ok(_) => Json(ApiResponse::ok_msg(if req.enable == 1 { "已启用" } else { "已禁用" })),
        Err(e) => {
            error!("设置用户状态失败: {}", e);
            Json(ApiResponse::ok_msg(&format!("操作失败: {}", e)))
        }
    }
}

// ===== 管理员注册用户 =====

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRegisterReq {
    pub user_name: String,
    pub password: String,
    pub nick_name: Option<String>,
    pub authority_id: Option<i64>,
    #[serde(default)]
    pub authority_ids: Vec<i64>,
    pub header_img: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

/// POST /user/admin_register
/// 管理员创建用户（需要认证），对应 Gin-Vue-Admin 的 userApi.Register()
pub async fn admin_register(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<AdminRegisterReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;

    let nick_name = req.nick_name.unwrap_or_else(|| req.user_name.clone());
    let authority_id = req.authority_id
        .or_else(|| req.authority_ids.first().copied())
        .unwrap_or(888);

    match UserService::register(db, &req.user_name, &req.password, &nick_name, authority_id).await {
        Ok(user) => Json(ApiResponse::ok_with_data(serde_json::json!({
            "user": {
                "ID": user.id,
                "uuid": user.uuid.to_string(),
                "userName": user.username,
                "nickName": user.nick_name,
                "authorityId": user.authority_id,
            }
        }), "创建成功")),
        Err(e) => {
            error!("管理员创建用户失败: {}", e);
            Json(ApiResponse::err_default(7008, &format!("创建失败: {}", e)))
        }
    }
}
