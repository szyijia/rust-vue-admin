pub mod health;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::warn;

use crate::{api, global::AppState, middleware as mw};

/// 自定义 JSON 提取错误处理
/// 将 axum 的反序列化错误（422）转为统一的 JSON 格式，并记录日志
pub async fn handle_json_rejection(rejection: JsonRejection) -> Response {
    let msg = rejection.body_text();
    warn!("JSON 请求体解析失败: {}", msg);
    let body = serde_json::json!({
        "code": 7400,
        "msg": format!("请求参数错误: {}", msg),
        "data": null
    });
    (StatusCode::OK, axum::Json(body)).into_response()
}

/// 注册所有路由，对应 Gin-Vue-Admin 的 initialize.Routers()
pub fn init_router(state: AppState) -> Router {
    let cors_layer = mw::build_cors_layer(&state.config.cors);

    // 公开路由（无需认证）
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/base/captcha", get(api::base::captcha))
        .route("/base/login", post(api::base::login))
        .route("/base/register", post(api::base::register))
        // 数据库初始化接口（对应 Gin-Vue-Admin 的 /init/）
        .route("/init/checkdb", get(api::init::check_db))
        .route("/init/initdb", post(api::init::init_db));

    // 私有路由（需要 JWT 认证 + Casbin 权限）
    let private_routes = Router::new()
        // 用户接口（个人操作）
        .route("/user/getUserInfo", get(api::user::get_user_info))
        .route("/user/setSelfInfo", put(api::user::set_self_info))
        .route("/user/changePassword", post(api::user::change_password))
        // JWT 黑名单（登出）
        .route("/jwt/jsonInBlacklist", post(api::user::json_in_blacklist))
        // 用户管理（管理员操作）
        .route(
            "/user/getUserList",
            post(api::system::sys_user::get_user_list),
        )
        .route(
            "/user/deleteUser",
            delete(api::system::sys_user::delete_user),
        )
        .route(
            "/user/setUserAuthority",
            post(api::system::sys_user::set_user_authority),
        )
        .route(
            "/user/setUserInfo",
            put(api::system::sys_user::set_user_info),
        )
        .route(
            "/user/resetPassword",
            post(api::system::sys_user::reset_password),
        )
        .route(
            "/user/setUserEnable",
            post(api::system::sys_user::set_user_enable),
        )
        .route(
            "/user/admin_register",
            post(api::system::sys_user::admin_register),
        )
        // 角色管理
        .route(
            "/authority/createAuthority",
            post(api::system::sys_authority::create_authority),
        )
        .route(
            "/authority/deleteAuthority",
            post(api::system::sys_authority::delete_authority),
        )
        .route(
            "/authority/updateAuthority",
            put(api::system::sys_authority::update_authority),
        )
        .route(
            "/authority/getAuthorityList",
            post(api::system::sys_authority::get_authority_list),
        )
        // Casbin 权限管理
        .route(
            "/casbin/UpdateCasbin",
            post(api::system::sys_authority::update_casbin),
        )
        .route(
            "/casbin/getPolicyPathByAuthorityId",
            post(api::system::sys_authority::get_policy_path_by_authority_id),
        )
        // 菜单管理
        .route("/menu/getMenu", post(api::system::sys_menu::get_menu))
        .route(
            "/menu/getMenuList",
            post(api::system::sys_menu::get_menu_list),
        )
        .route(
            "/menu/getBaseMenuTree",
            post(api::system::sys_menu::get_base_menu_tree),
        )
        .route(
            "/menu/getBaseMenuById",
            post(api::system::sys_menu::get_base_menu_by_id),
        )
        .route(
            "/menu/addBaseMenu",
            post(api::system::sys_menu::add_base_menu),
        )
        .route(
            "/menu/deleteBaseMenu",
            post(api::system::sys_menu::delete_base_menu),
        )
        .route(
            "/menu/updateBaseMenu",
            post(api::system::sys_menu::update_base_menu),
        )
        .route(
            "/menu/addMenuAuthority",
            post(api::system::sys_menu::add_menu_authority),
        )
        .route(
            "/menu/getMenuAuthority",
            post(api::system::sys_menu::get_menu_authority),
        )
        // API 管理
        .route("/api/createApi", post(api::system::sys_api::create_api))
        .route("/api/deleteApi", post(api::system::sys_api::delete_api))
        .route(
            "/api/deleteApisByIds",
            delete(api::system::sys_api::delete_apis_by_ids),
        )
        .route("/api/updateApi", post(api::system::sys_api::update_api))
        .route("/api/getApiList", post(api::system::sys_api::get_api_list))
        .route("/api/getAllApis", post(api::system::sys_api::get_all_apis))
        .route("/api/getApiById", post(api::system::sys_api::get_api_by_id))
        .route("/api/getApiGroups", get(api::system::sys_api::get_api_groups))
        // 先做 JWT 认证，再做 Casbin 鉴权
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw::casbin_auth,
        ))
        .layer(middleware::from_fn_with_state(state.clone(), mw::jwt_auth));

    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw::ip_rate_limit,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .with_state(state)
}
