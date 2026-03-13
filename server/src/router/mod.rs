pub mod health;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
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
    let cors_layer = mw::build_cors_layer(&state.get_config().cors);

    // 公开路由（无需认证）
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
.route("/base/captcha", post(api::base::captcha))
        .route("/base/login", post(api::base::login))
        .route("/base/register", post(api::base::register))
        // 数据库初始化接口（对应 Gin-Vue-Admin 的 /init/）
        .route("/init/checkdb", post(api::init::check_db))
        .route("/init/initdb", post(api::init::init_db))
        // 错误日志（无需认证，前端 error-handler 自动调用）
        .route("/sysError/createSysError", post(api::system::sys_error::create_sys_error));

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
        .route(
            "/user/setUserAuthorities",
            post(api::system::sys_user::set_user_authorities),
        )
        .route(
            "/user/setSelfSetting",
            put(api::system::sys_user::set_self_setting),
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
        .route(
            "/authority/copyAuthority",
            post(api::system::sys_authority::copy_authority),
        )
        .route(
            "/authority/setDataAuthority",
            post(api::system::sys_authority::set_data_authority),
        )
        .route(
            "/authority/setRoleUsers",
            post(api::system::sys_authority::set_role_users),
        )
        .route(
            "/authority/getUsersByAuthority",
            get(api::system::sys_authority::get_users_by_authority),
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
        .route(
            "/menu/getMenuRoles",
            get(api::system::sys_menu::get_menu_roles),
        )
        .route(
            "/menu/setMenuRoles",
            post(api::system::sys_menu::set_menu_roles),
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
        .route("/api/syncApi", get(api::system::sys_api::sync_api))
        .route("/api/ignoreApi", post(api::system::sys_api::ignore_api))
        .route("/api/enterSyncApi", post(api::system::sys_api::enter_sync_api))
        .route("/api/getApiRoles", get(api::system::sys_api::get_api_roles))
        .route("/api/setApiRoles", post(api::system::sys_api::set_api_roles))
        .route("/api/freshCasbin", get(api::system::sys_api::fresh_casbin))
        // 字典管理
        .route(
            "/sysDictionary/createSysDictionary",
            post(api::system::sys_dictionary::create_sys_dictionary),
        )
        .route(
            "/sysDictionary/deleteSysDictionary",
            delete(api::system::sys_dictionary::delete_sys_dictionary),
        )
        .route(
            "/sysDictionary/updateSysDictionary",
            put(api::system::sys_dictionary::update_sys_dictionary),
        )
        .route(
            "/sysDictionary/findSysDictionary",
            get(api::system::sys_dictionary::find_sys_dictionary),
        )
        .route(
            "/sysDictionary/getSysDictionaryList",
            get(api::system::sys_dictionary::get_sys_dictionary_list),
        )
        .route(
            "/sysDictionary/exportSysDictionary",
            get(api::system::sys_dictionary::export_sys_dictionary),
        )
        .route(
            "/sysDictionary/importSysDictionary",
            post(api::system::sys_dictionary::import_sys_dictionary),
        )
        // 字典详情
        .route(
            "/sysDictionaryDetail/createSysDictionaryDetail",
            post(api::system::sys_dictionary_detail::create_sys_dictionary_detail),
        )
        .route(
            "/sysDictionaryDetail/deleteSysDictionaryDetail",
            delete(api::system::sys_dictionary_detail::delete_sys_dictionary_detail),
        )
        .route(
            "/sysDictionaryDetail/updateSysDictionaryDetail",
            put(api::system::sys_dictionary_detail::update_sys_dictionary_detail),
        )
        .route(
            "/sysDictionaryDetail/findSysDictionaryDetail",
            get(api::system::sys_dictionary_detail::find_sys_dictionary_detail),
        )
        .route(
            "/sysDictionaryDetail/getSysDictionaryDetailList",
            get(api::system::sys_dictionary_detail::get_sys_dictionary_detail_list),
        )
        .route(
            "/sysDictionaryDetail/getDictionaryTreeList",
            get(api::system::sys_dictionary_detail::get_dictionary_tree_list),
        )
        .route(
            "/sysDictionaryDetail/getDictionaryTreeListByType",
            get(api::system::sys_dictionary_detail::get_dictionary_tree_list_by_type),
        )
        .route(
            "/sysDictionaryDetail/getDictionaryDetailsByParent",
            get(api::system::sys_dictionary_detail::get_dictionary_details_by_parent),
        )
        .route(
            "/sysDictionaryDetail/getDictionaryPath",
            get(api::system::sys_dictionary_detail::get_dictionary_path),
        )
        // 系统配置
        .route(
            "/system/getSystemConfig",
            post(api::system::sys_system::get_system_config),
        )
        .route(
            "/system/setSystemConfig",
            post(api::system::sys_system::set_system_config),
        )
        .route(
            "/system/getServerInfo",
            post(api::system::sys_system::get_server_info),
        )
        .route(
            "/system/reloadSystem",
            post(api::system::sys_system::reload_system),
        )
        // 邮件
        .route(
            "/email/emailTest",
            post(api::system::sys_email::email_test),
        )
        // 操作记录
        .route(
            "/sysOperationRecord/getSysOperationRecordList",
            get(api::system::sys_operation_record::get_sys_operation_record_list),
        )
        .route(
            "/sysOperationRecord/deleteSysOperationRecord",
            delete(api::system::sys_operation_record::delete_sys_operation_record),
        )
        .route(
            "/sysOperationRecord/deleteSysOperationRecordByIds",
            delete(api::system::sys_operation_record::delete_sys_operation_record_by_ids),
        )
        .route(
            "/sysOperationRecord/findSysOperationRecord",
            get(api::system::sys_operation_record::find_sys_operation_record),
        )
        // 参数管理
        .route(
            "/sysParams/createSysParams",
            post(api::system::sys_params::create_sys_params),
        )
        .route(
            "/sysParams/deleteSysParams",
            delete(api::system::sys_params::delete_sys_params),
        )
        .route(
            "/sysParams/deleteSysParamsByIds",
            delete(api::system::sys_params::delete_sys_params_by_ids),
        )
        .route(
            "/sysParams/updateSysParams",
            put(api::system::sys_params::update_sys_params),
        )
        .route(
            "/sysParams/findSysParams",
            get(api::system::sys_params::find_sys_params),
        )
        .route(
            "/sysParams/getSysParamsList",
            get(api::system::sys_params::get_sys_params_list),
        )
        .route(
            "/sysParams/getSysParam",
            get(api::system::sys_params::get_sys_params_by_key),
        )
        // 按钮权限
        .route(
            "/authorityBtn/getAuthorityBtn",
            post(api::system::sys_authority_btn::get_authority_btn),
        )
        .route(
            "/authorityBtn/setAuthorityBtn",
            post(api::system::sys_authority_btn::set_authority_btn),
        )
        .route(
            "/authorityBtn/canRemoveAuthorityBtn",
            post(api::system::sys_authority_btn::can_remove_authority_btn),
        )
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
        .layer(middleware::from_fn(mw::request_log))
        .layer(cors_layer)
        .with_state(state)
}
