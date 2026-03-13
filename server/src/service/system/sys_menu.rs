use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::{sys_menu, sys_authority_menu, sys_base_menu_btn, sys_base_menu_parameter, sys_role};

/// 菜单 meta 信息（与 Gin-Vue-Admin 前端格式兼容）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuMeta {
    pub active_name: String,
    pub title: String,
    pub icon: String,
    pub keep_alive: bool,
    pub default_menu: bool,
    pub close_tab: bool,
    pub transition_type: String,
}

/// 菜单按钮信息（与 Gin-Vue-Admin 前端格式兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuBtnInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,
    pub desc: String,
    #[serde(rename = "sysBaseMenuID")]
    pub sys_base_menu_id: u64,
}

/// 菜单参数信息（与 Gin-Vue-Admin 前端格式兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuParamInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    pub sys_base_menu_id: u64,
    #[serde(rename = "type")]
    pub param_type: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// 菜单信息（含子菜单，用于树形结构）
/// 与 Gin-Vue-Admin 前端格式兼容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "parentId")]
    pub parent_id: u64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i64,
    pub meta: MenuMeta,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<MenuInfo>,
    // 保留 btns 字段（前端 pinia/router.js 中使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btns: Option<Vec<String>>,
    /// 菜单关联的按钮列表（前端 menus.vue 中使用 data.menuBtn）
    #[serde(rename = "menuBtn", default)]
    pub menu_btn: Vec<MenuBtnInfo>,
    /// 菜单关联的参数列表（前端 menus.vue 中使用 data.parameters）
    #[serde(default)]
    pub parameters: Vec<MenuParamInfo>,
}

impl From<sys_menu::Model> for MenuInfo {
    fn from(m: sys_menu::Model) -> Self {
        Self {
            id: m.id,
            created_at: m.created_at,
            updated_at: m.updated_at,
            parent_id: m.parent_id,
            path: m.path,
            name: m.name,
            hidden: m.hidden,
            component: m.component,
            sort: m.sort,
            meta: MenuMeta {
                active_name: m.active_name.unwrap_or_default(),
                title: m.title,
                icon: m.icon,
                keep_alive: m.keep_alive,
                default_menu: m.default_menu,
                close_tab: m.close_tab,
                transition_type: m.transition_type.unwrap_or_default(),
            },
            children: vec![],
            btns: None,
            menu_btn: vec![],
            parameters: vec![],
        }
    }
}

/// 创建菜单请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMenuReq {
    pub parent_id: u64,
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub hidden: bool,
    pub component: String,
    #[serde(default)]
    pub sort: i64,
    #[serde(default)]
    pub active_name: String,
    #[serde(default)]
    pub keep_alive: bool,
    #[serde(default)]
    pub default_menu: bool,
    pub title: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub close_tab: bool,
    #[serde(default)]
    pub transition_type: String,
}

/// 加载所有菜单按钮，并按菜单ID分组
async fn load_menu_btns(db: &DatabaseConnection) -> Result<std::collections::HashMap<u64, Vec<MenuBtnInfo>>> {
    let btns = sys_base_menu_btn::Entity::find()
        .all(db)
        .await?;
    let mut map: std::collections::HashMap<u64, Vec<MenuBtnInfo>> = std::collections::HashMap::new();
    for btn in btns {
        map.entry(btn.sys_base_menu_id).or_default().push(MenuBtnInfo {
            id: btn.id,
            name: btn.name,
            desc: btn.desc,
            sys_base_menu_id: btn.sys_base_menu_id,
        });
    }
    Ok(map)
}

/// 为菜单列表填充 menuBtn 字段
fn fill_menu_btns(menus: &mut [MenuInfo], btn_map: &std::collections::HashMap<u64, Vec<MenuBtnInfo>>) {
    for menu in menus.iter_mut() {
        if let Some(btns) = btn_map.get(&menu.id) {
            menu.menu_btn = btns.clone();
        }
        if !menu.children.is_empty() {
            fill_menu_btns(&mut menu.children, btn_map);
        }
    }
}

/// 加载所有菜单参数，并按菜单ID分组
async fn load_menu_params(db: &DatabaseConnection) -> Result<std::collections::HashMap<u64, Vec<MenuParamInfo>>> {
    let params = sys_base_menu_parameter::Entity::find()
        .all(db)
        .await?;
    let mut map: std::collections::HashMap<u64, Vec<MenuParamInfo>> = std::collections::HashMap::new();
    for p in params {
        map.entry(p.sys_base_menu_id).or_default().push(MenuParamInfo {
            id: p.id,
            sys_base_menu_id: p.sys_base_menu_id,
            param_type: p.r#type.unwrap_or_default(),
            key: p.key.unwrap_or_default(),
            value: p.value.unwrap_or_default(),
            created_at: p.created_at,
            updated_at: p.updated_at,
        });
    }
    Ok(map)
}

/// 为菜单列表填充 parameters 字段
fn fill_menu_params(menus: &mut [MenuInfo], param_map: &std::collections::HashMap<u64, Vec<MenuParamInfo>>) {
    for menu in menus.iter_mut() {
        if let Some(params) = param_map.get(&menu.id) {
            menu.parameters = params.clone();
        }
        if !menu.children.is_empty() {
            fill_menu_params(&mut menu.children, param_map);
        }
    }
}

/// 获取所有菜单（平铺列表，内部使用）
pub async fn get_menu_list(db: &DatabaseConnection) -> Result<Vec<MenuInfo>> {
    let list = sys_menu::Entity::find()
        .order_by_asc(sys_menu::Column::Sort)
        .all(db)
        .await?;
    let btn_map = load_menu_btns(db).await?;
    let mut menus: Vec<MenuInfo> = list.into_iter().map(MenuInfo::from).collect();
    fill_menu_btns(&mut menus, &btn_map);
    Ok(menus)
}

/// 获取菜单管理页面的菜单列表（树形结构），对应 Go 端 GetInfoList / getBaseMenuTreeMap
/// 支持 UseStrictAuth 严格树角色模式：当开启时，非顶级角色只能看到自己有权限的菜单
pub async fn get_info_list(db: &DatabaseConnection, authority_id: u64, use_strict_auth: bool) -> Result<Vec<MenuInfo>> {
    // 1. 检查是否需要按角色过滤菜单（与 Go 端 getBaseMenuTreeMap 逻辑一致）
    let need_filter = if use_strict_auth {
        // 获取父角色ID
        let authority = sys_role::Entity::find()
            .filter(sys_role::Column::AuthorityId.eq(authority_id))
            .one(db)
            .await?;
        match authority {
            Some(auth) => auth.parent_id != 0,
            None => false,
        }
    } else {
        false
    };

    // 2. 查询菜单列表
    let all_menus = if need_filter {
        // 严格模式且非顶级角色：只查询该角色有权限的菜单
        let authority_menus = sys_authority_menu::Entity::find()
            .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(authority_id))
            .all(db)
            .await?;
        let menu_ids: Vec<u64> = authority_menus.into_iter().map(|am| am.sys_base_menu_id).collect();
        if menu_ids.is_empty() {
            return Ok(vec![]);
        }
        sys_menu::Entity::find()
            .filter(sys_menu::Column::Id.is_in(menu_ids))
            .order_by_asc(sys_menu::Column::Sort)
            .all(db)
            .await?
    } else {
        // 非严格模式或顶级角色：查询所有菜单
        sys_menu::Entity::find()
            .order_by_asc(sys_menu::Column::Sort)
            .all(db)
            .await?
    };

    // 3. 加载关联数据
    let btn_map = load_menu_btns(db).await?;
    let param_map = load_menu_params(db).await?;

    // 4. 转换为 MenuInfo 并构建树形结构
    let menus: Vec<MenuInfo> = all_menus.into_iter().map(MenuInfo::from).collect();
    let mut tree = build_menu_tree(menus, 0);
    fill_menu_btns(&mut tree, &btn_map);
    fill_menu_params(&mut tree, &param_map);
    Ok(tree)
}

/// 获取菜单树（根据角色 ID 过滤）
/// 与 gin-vue-admin 逻辑一致：始终通过 sys_authority_menus 关联表查询该角色绑定的菜单
pub async fn get_menu_tree(db: &DatabaseConnection, role_id: u64) -> Result<Vec<MenuInfo>> {
    // 1. 查询该角色绑定的菜单 ID
    let authority_menus = sys_authority_menu::Entity::find()
        .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(role_id))
        .all(db)
        .await?;

    let menu_ids: Vec<u64> = authority_menus
        .into_iter()
        .map(|am| am.sys_base_menu_id)
        .collect();

    if menu_ids.is_empty() {
        return Ok(vec![]);
    }

    // 2. 根据菜单 ID 查询菜单详情
    // sys_base_menu_id 是 u64，sys_base_menus.id 也是 u64
    let all_menus = sys_menu::Entity::find()
        .filter(sys_menu::Column::Id.is_in(menu_ids.clone()))
        .order_by_asc(sys_menu::Column::Sort)
        .all(db)
        .await?;

    let btn_map = load_menu_btns(db).await?;
    let menus: Vec<MenuInfo> = all_menus.into_iter().map(MenuInfo::from).collect();
    let mut tree = build_menu_tree(menus, 0);
    fill_menu_btns(&mut tree, &btn_map);
    Ok(tree)
}

/// 将平铺菜单列表构建为树形结构
fn build_menu_tree(menus: Vec<MenuInfo>, parent_id: u64) -> Vec<MenuInfo> {
    menus
        .iter()
        .filter(|m| m.parent_id == parent_id)
        .map(|m| {
            let mut node = m.clone();
            node.children = build_menu_tree(menus.clone(), m.id);
            node
        })
        .collect()
}

/// 新增菜单
pub async fn add_base_menu(db: &DatabaseConnection, req: CreateMenuReq) -> Result<()> {
    let model = sys_menu::ActiveModel {
        parent_id: Set(req.parent_id),
        path: Set(req.path),
        name: Set(req.name),
        hidden: Set(req.hidden),
        component: Set(req.component),
        sort: Set(req.sort),
        active_name: Set(Some(req.active_name)),
        keep_alive: Set(req.keep_alive),
        default_menu: Set(req.default_menu),
        title: Set(req.title),
        icon: Set(req.icon),
        close_tab: Set(req.close_tab),
        transition_type: Set(Some(req.transition_type)),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 删除菜单
pub async fn delete_base_menu(db: &DatabaseConnection, id: u64) -> Result<()> {
    let children = sys_menu::Entity::find()
        .filter(sys_menu::Column::ParentId.eq(id))
        .one(db)
        .await?;
    if children.is_some() {
        return Err(anyhow::anyhow!("该菜单下存在子菜单，请先删除子菜单"));
    }
    sys_menu::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 更新菜单
pub async fn update_base_menu(db: &DatabaseConnection, id: u64, req: CreateMenuReq) -> Result<()> {
    let model = sys_menu::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("菜单不存在"))?;

    let mut active: sys_menu::ActiveModel = model.into();
    active.parent_id = Set(req.parent_id);
    active.path = Set(req.path);
    active.name = Set(req.name);
    active.hidden = Set(req.hidden);
    active.component = Set(req.component);
    active.sort = Set(req.sort);
    active.active_name = Set(Some(req.active_name));
    active.keep_alive = Set(req.keep_alive);
    active.default_menu = Set(req.default_menu);
    active.title = Set(req.title);
    active.icon = Set(req.icon);
    active.close_tab = Set(req.close_tab);
    active.transition_type = Set(Some(req.transition_type));
    active.update(db).await?;
    Ok(())
}

/// 设置角色菜单关联
pub async fn add_menu_authority(
    db: &DatabaseConnection,
    menu_ids: Vec<u64>,
    role_id: u64,
) -> Result<()> {
    // 先删除该角色的所有菜单关联
    sys_authority_menu::Entity::delete_many()
        .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(role_id))
        .exec(db)
        .await?;

    // 重新插入
    for menu_id in menu_ids {
        let model = sys_authority_menu::ActiveModel {
            sys_base_menu_id: Set(menu_id),
            sys_authority_authority_id: Set(role_id),
        };
        model.insert(db).await?;
    }
    Ok(())
}

/// 获取角色的菜单 ID 列表
pub async fn get_menu_authority(db: &DatabaseConnection, role_id: u64) -> Result<Vec<u64>> {
    let list = sys_authority_menu::Entity::find()
        .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(role_id))
        .all(db)
        .await?;
    Ok(list.into_iter().map(|m| m.sys_base_menu_id).collect())
}

/// 根据 ID 获取单个菜单
pub async fn get_base_menu_by_id(db: &DatabaseConnection, id: u64) -> Result<Option<MenuInfo>> {
    let model = sys_menu::Entity::find_by_id(id).one(db).await?;
    Ok(model.map(MenuInfo::from))
}

/// 获取拥有指定菜单的所有角色ID，对应 Gin-Vue-Admin 的 menuService.GetAuthoritiesByMenuId()
pub async fn get_authorities_by_menu_id(db: &DatabaseConnection, menu_id: u64) -> Result<Vec<u64>> {
    let records = sys_authority_menu::Entity::find()
        .filter(sys_authority_menu::Column::SysBaseMenuId.eq(menu_id))
        .all(db)
        .await?;
    Ok(records.into_iter().map(|r| r.sys_authority_authority_id).collect())
}

/// 获取将指定菜单设为首页的角色ID列表，对应 Gin-Vue-Admin 的 menuService.GetDefaultRouterAuthorityIds()
pub async fn get_default_router_authority_ids(db: &DatabaseConnection, menu_id: u64) -> Result<Vec<u64>> {
    use crate::model::system::sys_role;

    // 先找到菜单的 name
    let menu = sys_menu::Entity::find_by_id(menu_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("菜单不存在"))?;

    // 查找 default_router 等于该菜单 name 的角色
    let roles = sys_role::Entity::find()
        .filter(sys_role::Column::DefaultRouter.eq(&menu.name))
        .all(db)
        .await?;

    Ok(roles.into_iter().map(|r| r.authority_id).collect())
}

/// 全量覆盖某菜单关联的角色列表，对应 Gin-Vue-Admin 的 menuService.SetMenuAuthorities()
pub async fn set_menu_authorities(
    db: &DatabaseConnection,
    menu_id: u64,
    authority_ids: Vec<u64>,
) -> Result<()> {
    use sea_orm::TransactionTrait;
    let txn = db.begin().await?;

    // 1. 删除该菜单所有已有的角色关联
    sys_authority_menu::Entity::delete_many()
        .filter(sys_authority_menu::Column::SysBaseMenuId.eq(menu_id))
        .exec(&txn)
        .await?;

    // 2. 批量插入新的关联记录
    for authority_id in authority_ids {
        let model = sys_authority_menu::ActiveModel {
            sys_base_menu_id: Set(menu_id),
            sys_authority_authority_id: Set(authority_id),
        };
        model.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(())
}
