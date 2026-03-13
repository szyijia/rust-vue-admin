use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    global::{response::ApiResponse, AppState},
    service::system::{sys_menu, sys_menu::MenuInfo},
    utils::jwt::Claims,
};

#[derive(Debug, Deserialize)]
pub struct DeleteMenuReq {
    #[serde(alias = "id", alias = "ID")]
    pub id: u64,
}

/// 前端发送的菜单更新请求（兼容 Gin-Vue-Admin 前端格式）
/// 前端发送: { ID: 1, parentId: 0, path: '...', name: '...', meta: { title, icon, keepAlive, ... }, ... }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenuReq {
    #[serde(alias = "id", alias = "ID")]
    pub id: u64,
    pub parent_id: u64,
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub hidden: bool,
    pub component: String,
    #[serde(default)]
    pub sort: i64,
    #[serde(default)]
    pub meta: UpdateMenuMeta,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenuMeta {
    #[serde(default)]
    pub active_name: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub keep_alive: bool,
    #[serde(default)]
    pub default_menu: bool,
    #[serde(default)]
    pub close_tab: bool,
    #[serde(default)]
    pub transition_type: String,
}

/// 前端发送的菜单新增请求（兼容 Gin-Vue-Admin 前端的嵌套 meta 格式）
/// 前端发送: { ID: 0, parentId: 0, path: '...', name: '...', meta: { title, icon, keepAlive, ... }, menuBtn: [], parameters: [] }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMenuReq {
    #[serde(alias = "id", alias = "ID", default)]
    pub id: u64,
    #[serde(default)]
    pub parent_id: u64,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub component: String,
    #[serde(default)]
    pub sort: i64,
    #[serde(default)]
    pub meta: AddMenuMeta,
    #[serde(default)]
    pub menu_btn: Vec<serde_json::Value>,
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddMenuMeta {
    #[serde(default)]
    pub active_name: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub keep_alive: bool,
    #[serde(default)]
    pub default_menu: bool,
    #[serde(default)]
    pub close_tab: bool,
    #[serde(default)]
    pub transition_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMenuAuthorityReq {
    pub authority_id: u64,
    pub menus: Vec<MenuIdReq>,
}

#[derive(Debug, Deserialize)]
pub struct MenuIdReq {
    #[serde(alias = "ID")]
    pub id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMenuAuthorityReq { pub authority_id: u64 }

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenusResponse { pub menus: Vec<MenuInfo> }

/// POST /menu/getMenu
pub async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Json<ApiResponse<MenusResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_menu::get_menu_tree(db, claims.role_id).await {
        Ok(menus) => Json(ApiResponse::ok_with_data(MenusResponse { menus }, "获取成功")),
        Err(e) => { error!("获取菜单树失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /menu/getMenuList
pub async fn get_menu_list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Json<ApiResponse<Vec<MenuInfo>>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    let use_strict_auth = state.get_config().system.use_strict_auth;
    match sys_menu::get_info_list(db, claims.role_id, use_strict_auth).await {
        Ok(list) => Json(ApiResponse::ok_with_data(list, "获取成功")),
        Err(e) => { error!("获取菜单列表失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /menu/addBaseMenu
pub async fn add_base_menu(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<AddMenuReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    // 将前端嵌套 meta 格式转换为后端平铺的 CreateMenuReq
    let data = sys_menu::CreateMenuReq {
        parent_id: req.parent_id,
        path: req.path,
        name: req.name,
        hidden: req.hidden,
        component: req.component,
        sort: req.sort,
        active_name: req.meta.active_name,
        keep_alive: req.meta.keep_alive,
        default_menu: req.meta.default_menu,
        title: req.meta.title,
        icon: req.meta.icon,
        close_tab: req.meta.close_tab,
        transition_type: req.meta.transition_type,
    };
    match sys_menu::add_base_menu(db, data).await {
        Ok(_) => Json(ApiResponse::ok_msg("添加成功")),
        Err(e) => { error!("新增菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("添加失败: {}", e))) }
    }
}

/// POST /menu/deleteBaseMenu
pub async fn delete_base_menu(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<DeleteMenuReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_menu::delete_base_menu(db, req.id).await {
        Ok(_) => Json(ApiResponse::ok_msg("删除成功")),
        Err(e) => { error!("删除菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("删除失败: {}", e))) }
    }
}

/// PUT /menu/updateBaseMenu
pub async fn update_base_menu(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<UpdateMenuReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    // 将前端嵌套 meta 格式转换为后端平铺的 CreateMenuReq
    let data = sys_menu::CreateMenuReq {
        parent_id: req.parent_id,
        path: req.path,
        name: req.name,
        hidden: req.hidden,
        component: req.component,
        sort: req.sort,
        active_name: req.meta.active_name,
        keep_alive: req.meta.keep_alive,
        default_menu: req.meta.default_menu,
        title: req.meta.title,
        icon: req.meta.icon,
        close_tab: req.meta.close_tab,
        transition_type: req.meta.transition_type,
    };
    match sys_menu::update_base_menu(db, req.id, data).await {
        Ok(_) => Json(ApiResponse::ok_msg("更新成功")),
        Err(e) => { error!("更新菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("更新失败: {}", e))) }
    }
}

/// POST /menu/addMenuAuthority
pub async fn add_menu_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<AddMenuAuthorityReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    let menu_ids: Vec<u64> = req.menus.into_iter().map(|m| m.id).collect();
    match sys_menu::add_menu_authority(db, menu_ids, req.authority_id).await {
        Ok(_) => Json(ApiResponse::ok_msg("添加成功")),
        Err(e) => { error!("设置角色菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("添加失败: {}", e))) }
    }
}

/// POST /menu/getMenuAuthority
pub async fn get_menu_authority(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetMenuAuthorityReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_menu::get_menu_authority(db, req.authority_id).await {
        Ok(menu_ids) => {
            let all_menus = sys_menu::get_menu_list(db).await.unwrap_or_default();
            // 构造带 menuId 字段的返回数据，与 gin-vue-admin 的 SysMenu 格式一致
            // 前端 menus.vue 通过 item.menuId 和 same.parentId 来判断选中状态
            let menus: Vec<serde_json::Value> = all_menus.iter()
                .filter(|m| menu_ids.contains(&m.id))
                .map(|m| {
                    let mut obj = serde_json::to_value(m).unwrap_or_default();
                    if let Some(map) = obj.as_object_mut() {
                        map.insert("menuId".to_string(), serde_json::json!(m.id));
                    }
                    obj
                })
                .collect();
            Json(ApiResponse::ok_with_data(serde_json::json!({ "menus": menus }), "获取成功"))
        }
        Err(e) => { error!("获取角色菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /menu/getBaseMenuTree
/// 获取菜单树（所有菜单，用于菜单管理页面）
pub async fn get_base_menu_tree(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<MenusResponse>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    // 超级管理员可以看到所有菜单（包括隐藏的）
    match sys_menu::get_menu_tree(db, 888).await {
        Ok(menus) => Json(ApiResponse::ok_with_data(MenusResponse { menus }, "获取成功")),
        Err(e) => { error!("获取菜单树失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

/// POST /menu/getBaseMenuById
/// 根据 ID 获取单个菜单
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMenuByIdReq { pub id: u64 }

pub async fn get_base_menu_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<GetMenuByIdReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    match sys_menu::get_base_menu_by_id(db, req.id).await {
        Ok(Some(menu)) => Json(ApiResponse::ok_with_data(serde_json::json!({ "menu": menu }), "获取成功")),
        Ok(None) => Json(ApiResponse::err_default(7001, "菜单不存在")),
        Err(e) => { error!("获取菜单失败: {}", e); Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))) }
    }
}

// ===== 新增接口 =====

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMenuRolesReq {
    pub menu_id: u64,
}

/// GET /menu/getMenuRoles
/// 获取拥有指定菜单的角色ID列表，对应 Gin-Vue-Admin 的 AuthorityMenuApi.GetMenuRoles()
pub async fn get_menu_roles(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    axum::extract::Query(req): axum::extract::Query<GetMenuRolesReq>,
) -> Json<ApiResponse<serde_json::Value>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::err_default(7005, "数据库未连接"));
    };
    let db = &*db;
    if req.menu_id == 0 {
        return Json(ApiResponse::err_default(7001, "菜单ID不能为空"));
    }
    let authority_ids = match sys_menu::get_authorities_by_menu_id(db, req.menu_id).await {
        Ok(ids) => ids,
        Err(e) => { error!("获取失败: {}", e); return Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))); }
    };
    let default_router_authority_ids = match sys_menu::get_default_router_authority_ids(db, req.menu_id).await {
        Ok(ids) => ids,
        Err(e) => { error!("获取首页角色失败: {}", e); return Json(ApiResponse::err_default(7001, &format!("获取失败: {}", e))); }
    };
    Json(ApiResponse::ok_with_data(serde_json::json!({
        "authorityIds": authority_ids,
        "defaultRouterAuthorityIds": default_router_authority_ids,
    }), "获取成功"))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMenuRolesReq {
    pub menu_id: u64,
    #[serde(default)]
    pub authority_ids: Vec<u64>,
}

/// POST /menu/setMenuRoles
/// 全量覆盖某菜单关联的角色列表，对应 Gin-Vue-Admin 的 AuthorityMenuApi.SetMenuRoles()
pub async fn set_menu_roles(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<SetMenuRolesReq>,
) -> Json<ApiResponse<()>> {
    let Some(db) = state.try_get_db() else {
        return Json(ApiResponse::ok_msg("数据库未连接"));
    };
    let db = &*db;
    if req.menu_id == 0 {
        return Json(ApiResponse::ok_msg("菜单ID不能为空"));
    }
    match sys_menu::set_menu_authorities(db, req.menu_id, req.authority_ids).await {
        Ok(_) => Json(ApiResponse::ok_msg("设置成功")),
        Err(e) => { error!("设置失败: {}", e); Json(ApiResponse::ok_msg(&format!("设置失败: {}", e))) }
    }
}
