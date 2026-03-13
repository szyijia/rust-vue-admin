use axum::extract::{Extension, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{ApiResponse, AppState},
    model::system::{sys_authority_menu, sys_jwt_blacklist, sys_menu},
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
    pub authority_id: u64,
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
    pub password: String,
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
            // 查询当前角色信息
            let authority = sys_authority::get_authority_by_id(db, user.authority_id)
                .await
                .ok()
                .flatten();

            // UserAuthorityDefaultRouter: 检查默认路由是否在该角色的菜单中
            let default_router = if let Some(ref auth) = authority {
                let menu_ids: Vec<u64> = sys_authority_menu::Entity::find()
                    .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(user.authority_id))
                    .all(db)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| m.sys_base_menu_id)
                    .collect();
                // 检查 default_router 对应的菜单是否在角色的菜单列表中
                let found = if !menu_ids.is_empty() {
                    sys_menu::Entity::find()
                        .filter(sys_menu::Column::Name.eq(auth.default_router.as_str()))
                        .filter(sys_menu::Column::Id.is_in(menu_ids))
                        .one(db)
                        .await
                        .unwrap_or(None)
                } else {
                    None
                };
                if found.is_none() {
                    "404".to_string()
                } else {
                    auth.default_router.clone()
                }
            } else {
                "dashboard".to_string()
            };

            // 查询多角色列表（authorities）
            let authority_ids = UserService::get_user_authorities(db, user.id)
                .await
                .unwrap_or_default();
            let mut authorities = Vec::new();
            for aid in &authority_ids {
                if let Ok(Some(a)) = sys_authority::get_authority_by_id(db, *aid).await {
                    authorities.push(serde_json::json!({
                        "authorityId": a.authority_id,
                        "authorityName": a.authority_name,
                        "parentId": a.parent_id,
                        "defaultRouter": a.default_router,
                    }));
                }
            }

            // 解析 originSetting（JSON 字符串 -> JSON 对象）
            let origin_setting: serde_json::Value = user.origin_setting
                .as_deref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(serde_json::Value::Null);

            ApiResponse::ok(serde_json::json!({
                "userInfo": {
                    "ID": user.id,
                    "CreatedAt": user.created_at,
                    "UpdatedAt": user.updated_at,
                    "uuid": user.uuid.to_string(),
                    "userName": user.username,
                    "nickName": user.nick_name,
                    "headerImg": user.header_img,
                    "phone": user.phone,
                    "email": user.email,
                    "enable": user.enable,
                    "authorityId": user.authority_id,
                    "authority": {
                        "authorityId": authority.as_ref().map(|a| a.authority_id).unwrap_or(user.authority_id),
                        "authorityName": authority.as_ref().map(|a| a.authority_name.as_str()).unwrap_or(""),
                        "defaultRouter": default_router,
                    },
                    "authorities": authorities,
                    "originSetting": origin_setting,
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

    match UserService::change_password(db, claims.user_id, &req.password, &req.new_password).await {
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
