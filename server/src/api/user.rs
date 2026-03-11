use axum::extract::{Extension, State};
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{ApiResponse, AppState},
    model::system::sys_jwt_blacklist,
    service::{system::sys_authority, UserService},
    utils::Claims,
};

/// 用户信息响应（脱敏）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponse {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub nick_name: String,
    pub header_img: String,
    pub phone: String,
    pub email: String,
    pub authority_id: i64,
}

/// 修改个人信息请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSelfInfoReq {
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub header_img: Option<String>,
}

/// 修改密码请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordReq {
    pub old_password: String,
    pub new_password: String,
}

/// 获取当前登录用户信息，对应 Gin-Vue-Admin 的 userApi.GetUserInfo()
/// GET /user/getUserInfo
pub async fn get_user_info(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResponse<serde_json::Value> {
    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    match UserService::find_by_id(db, claims.user_id).await {
        Ok(user) => {
            // 查询角色信息
            let authority = sys_authority::get_authority_by_id(db, user.authority_id)
                .await
                .ok()
                .flatten();

            ApiResponse::ok(serde_json::json!({
                "userInfo": {
                    "ID": user.id,
                    "uuid": user.uuid.to_string(),
                    "userName": user.username,
                    "nickName": user.nick_name,
                    "headerImg": user.header_img,
                    "phone": user.phone,
                    "email": user.email,
                    "authorityId": user.authority_id,
                    "authority": {
                        "authorityId": authority.as_ref().map(|a| a.authority_id).unwrap_or(user.authority_id),
                        "authorityName": authority.as_ref().map(|a| a.authority_name.as_str()).unwrap_or(""),
                        "defaultRouter": authority.as_ref().map(|a| a.default_router.as_str()).unwrap_or("dashboard"),
                    }
                }
            }))
        }
        Err(e) => ApiResponse::fail(7009, e.to_string(), serde_json::Value::Null),
    }
}

/// 修改当前用户个人信息，对应 Gin-Vue-Admin 的 userApi.SetSelfInfo()
/// PUT /user/setSelfInfo
pub async fn set_self_info(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::Json(req): axum::Json<UpdateSelfInfoReq>,
) -> ApiResponse<serde_json::Value> {
    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    match UserService::update_user_info(db, claims.user_id, req.nick_name, req.phone, req.email, req.header_img).await {
        Ok(user) => ApiResponse::ok(serde_json::json!({
            "userInfo": {
                "id": user.id,
                "uuid": user.uuid.to_string(),
                "username": user.username,
                "nick_name": user.nick_name,
                "header_img": user.header_img,
                "phone": user.phone,
                "email": user.email,
                "authority_id": user.authority_id,
            }
        })),
        Err(e) => {
            error!("修改个人信息失败: {}", e);
            ApiResponse::fail(7001, e.to_string(), serde_json::Value::Null)
        }
    }
}

/// 修改当前用户密码，对应 Gin-Vue-Admin 的 userApi.ChangePassword()
/// POST /user/changePassword
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::Json(req): axum::Json<ChangePasswordReq>,
) -> ApiResponse<serde_json::Value> {
    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    if req.new_password.len() < 6 {
        return ApiResponse::fail(7002, "新密码至少6位", serde_json::Value::Null);
    }

    match UserService::change_password(db, claims.user_id, &req.old_password, &req.new_password).await {
        Ok(_) => ApiResponse::ok(serde_json::json!({"msg": "密码修改成功"})),
        Err(e) => {
            error!("修改密码失败: {}", e);
            ApiResponse::fail(7001, e.to_string(), serde_json::Value::Null)
        }
    }
}

/// 将 JWT Token 加入黑名单（登出），对应 Gin-Vue-Admin 的 jwtApi.JsonInBlacklist()
/// POST /jwt/jsonInBlacklist
pub async fn json_in_blacklist(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::RawQuery(_): axum::extract::RawQuery,
    headers: axum::http::HeaderMap,
) -> ApiResponse<serde_json::Value> {
    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    // 从 x-token 或 Authorization: Bearer 头提取 token
    let token = headers
        .get("x-token")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
        })
        .unwrap_or("")
        .to_string();

    if token.is_empty() {
        return ApiResponse::ok(serde_json::json!({"msg": "拉黑成功"}));
    }

    let jwt_model = sys_jwt_blacklist::ActiveModel {
        jwt: Set(token),
        ..Default::default()
    };
    match jwt_model.insert(db).await {
        Ok(_) => {
            let _ = claims; // 使用 claims 避免 unused warning
            ApiResponse::ok(serde_json::json!({"msg": "拉黑成功"}))
        }
        Err(e) => {
            error!("JWT 黑名单写入失败: {}", e);
            // 即使写入失败也返回成功（前端会清除本地 token）
            ApiResponse::ok(serde_json::json!({"msg": "拉黑成功"}))
        }
    }
}
