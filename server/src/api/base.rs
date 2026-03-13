use axum::{extract::State, extract::rejection::JsonRejection, Json};
use serde::{Deserialize, Serialize};
use tracing::warn;
use validator::Validate;

use crate::{
    global::{ApiResponse, AppState},
    service::{system::sys_authority, UserService},
    utils::{
        captcha::{captcha_key, generate_captcha},
        create_token,
    },
};

// ===== 请求/响应结构体 =====

/// 登录请求，对应 Gin-Vue-Admin 的 request.Login
/// 前端发送 camelCase 字段：captchaId、openCaptcha 等
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 用户名
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    /// 密码
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
    /// 验证码 ID（前端字段名：captchaId）
    pub captcha_id: String,
    /// 验证码答案
    pub captcha: String,
    /// 是否开启验证码（前端字段名：openCaptcha，可选）
    pub open_captcha: Option<bool>,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: i64,
    pub user: UserInfo,
}

/// 用户信息（脱敏，不含密码）
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct UserInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    pub uuid: String,
    pub userName: String,
    pub nickName: String,
    pub headerImg: String,
    pub phone: String,
    pub email: String,
    pub authorityId: u64,
    pub authority: AuthorityInfo,
}

/// 角色信息（嵌套在用户信息中）
#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct AuthorityInfo {
    pub authorityId: u64,
    pub authorityName: String,
    pub defaultRouter: String,
}

/// 注册请求
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "用户名长度为3-20位"))]
    pub username: String,
    #[validate(length(min = 8, message = "密码至少8位"))]
    pub password: String,
    pub nick_name: Option<String>,
    pub authority_id: Option<u64>,
}

/// 验证码响应
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct CaptchaResponse {
    pub captchaId: String,
    pub picPath: String,
    pub captchaLength: i32,
    pub openCaptcha: bool,
}

// ===== Handler 函数 =====

/// 获取验证码，对应 Gin-Vue-Admin 的 baseApi.Captcha()
/// GET /base/captcha
pub async fn captcha(State(state): State<AppState>) -> ApiResponse<CaptchaResponse> {
    let config = state.get_config();
    let cfg = &config.captcha;

    // 生成验证码
    let result = match generate_captcha(cfg.img_width, cfg.img_height) {
        Ok(r) => r,
        Err(e) => {
            return ApiResponse::fail(7001, format!("验证码生成失败: {}", e), CaptchaResponse {
                captchaId: String::new(),
                picPath: String::new(),
                captchaLength: cfg.key_long as i32,
                openCaptcha: true,
            });
        }
    };

    // 将验证码答案存入 Redis（有 Redis 时）或内存（无 Redis 时）
    if let Some(redis) = &state.redis {
        let key = captcha_key(&result.captcha_id);
        let mut conn = redis.as_ref().clone();
        let _ = redis::cmd("SETEX")
            .arg(&key)
            .arg(cfg.open_captcha_timeout)
            .arg(&result.answer)
            .query_async::<()>(&mut conn)
            .await;
    }

    ApiResponse::ok(CaptchaResponse {
        captchaId: result.captcha_id,
        picPath: result.pic_path,
        captchaLength: cfg.key_long as i32,
        openCaptcha: cfg.open_captcha == 0,
    })
}

/// 用户登录，对应 Gin-Vue-Admin 的 baseApi.Login()
/// POST /base/login
pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> ApiResponse<serde_json::Value> {
    // 获取当前配置快照（支持热重载）
    let config = state.get_config();

    // 处理 JSON 反序列化错误
    let Json(req) = match payload {
        Ok(json) => json,
        Err(e) => {
            warn!("登录请求 JSON 解析失败: {}", e.body_text());
            return ApiResponse::fail(7400, format!("请求参数错误: {}", e.body_text()), serde_json::Value::Null);
        }
    };

    // 参数校验
    if let Err(e) = req.validate() {
        return ApiResponse::fail(7002, format!("参数错误: {}", e), serde_json::Value::Null);
    }

    // 验证码校验（open_captcha != 0 时开启验证码，有 Redis 时才校验）
    if config.captcha.open_captcha != 0 {
        if let Some(redis) = &state.redis {
            let key = captcha_key(&req.captcha_id);
            let mut conn = redis.as_ref().clone();
            let stored: Option<String> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            match stored {
                None => {
                    return ApiResponse::fail(7003, "验证码已过期，请重新获取", serde_json::Value::Null);
                }
                Some(answer) if answer.to_lowercase() != req.captcha.to_lowercase() => {
                    return ApiResponse::fail(7004, "验证码错误", serde_json::Value::Null);
                }
                _ => {
                    // 验证通过，删除验证码（一次性使用）
                    let _ = redis::cmd("DEL")
                        .arg(&key)
                        .query_async::<()>(&mut conn)
                        .await;
                }
            }
        }
    }

    // 获取数据库连接
    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化，请先访问 /init/initdb 初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    // 执行登录
    match UserService::login(db, &req.username, &req.password).await {
        Ok(user) => {
            // 生成 JWT Token
            match create_token(
                user.id,
                &user.username,
                user.authority_id,
                "",
                &config.jwt,
            ) {
                Ok(token_result) => {
                    // 查询角色信息
                    let authority = sys_authority::get_authority_by_id(db, user.authority_id)
                        .await
                        .ok()
                        .flatten()
                        .map(|a| AuthorityInfo {
                            authorityId: a.authority_id,
                            authorityName: a.authority_name,
                            defaultRouter: a.default_router,
                        })
                        .unwrap_or_default();

                    let user_info = UserInfo {
                        id: user.id,
                        uuid: user.uuid.to_string(),
                        userName: user.username,
                        nickName: user.nick_name,
                        headerImg: user.header_img,
                        phone: user.phone,
                        email: user.email,
                        authorityId: user.authority_id,
                        authority,
                    };
                    ApiResponse::ok(serde_json::json!({
                        "token": token_result.token,
                        "expiresAt": token_result.expires_at,
                        "user": user_info,
                    }))
                }
                Err(e) => ApiResponse::fail(7006, format!("Token 生成失败: {}", e), serde_json::Value::Null),
            }
        }
        Err(e) => ApiResponse::fail(7007, e.to_string(), serde_json::Value::Null),
    }
}

/// 用户注册，对应 Gin-Vue-Admin 的 baseApi.Register()
/// POST /base/register
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResponse<serde_json::Value> {
    // 参数校验
    if let Err(e) = req.validate() {
        return ApiResponse::fail(7002, format!("参数错误: {}", e), serde_json::Value::Null);
    }

    let db = match state.try_get_db() {
        Some(db) => db,
        None => {
            return ApiResponse::fail(7005, "数据库未初始化", serde_json::Value::Null);
        }
    };
    let db = db.as_ref();

    let nick_name = req.nick_name.unwrap_or_else(|| req.username.clone());
    let authority_id = req.authority_id.unwrap_or(888u64);

    match UserService::register(db, &req.username, &req.password, &nick_name, authority_id).await {
        Ok(user) => ApiResponse::ok(serde_json::json!({
            "id": user.id,
            "uuid": user.uuid.to_string(),
            "username": user.username,
            "nick_name": user.nick_name,
        })),
        Err(e) => ApiResponse::fail(7008, e.to_string(), serde_json::Value::Null),
    }
}
