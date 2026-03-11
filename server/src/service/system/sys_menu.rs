use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::{sys_menu, sys_authority_menu};

/// 菜单 meta 信息（与 Gin-Vue-Admin 前端格式兼容）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MenuMeta {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "keepAlive")]
    pub keep_alive: bool,
    #[serde(rename = "defaultMenu")]
    pub default_menu: bool,
    #[serde(rename = "closeTab")]
    pub close_tab: bool,
}

/// 菜单信息（含子菜单，用于树形结构）
/// 与 Gin-Vue-Admin 前端格式兼容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuInfo {
    pub id: i32,
    #[serde(rename = "parentId")]
    pub parent_id: i32,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i32,
    pub meta: MenuMeta,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<MenuInfo>,
    // 保留 btns 字段（前端 pinia/router.js 中使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btns: Option<Vec<String>>,
}

impl From<sys_menu::Model> for MenuInfo {
    fn from(m: sys_menu::Model) -> Self {
        Self {
            id: m.id,
            parent_id: m.parent_id,
            path: m.path,
            name: m.name,
            hidden: m.hidden,
            component: m.component,
            sort: m.sort,
            meta: MenuMeta {
                title: m.title,
                icon: m.icon,
                keep_alive: m.keep_alive,
                default_menu: m.default_menu,
                close_tab: m.close_tab,
            },
            children: vec![],
            btns: None,
        }
    }
}

/// 创建菜单请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMenuReq {
    pub parent_id: i32,
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub hidden: bool,
    pub component: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub keep_alive: bool,
    #[serde(default)]
    pub default_menu: bool,
    pub title: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub close_tab: bool,
}

/// 获取所有菜单（平铺列表）
pub async fn get_menu_list(db: &DatabaseConnection) -> Result<Vec<MenuInfo>> {
    let list = sys_menu::Entity::find()
        .order_by_asc(sys_menu::Column::Sort)
        .all(db)
        .await?;
    Ok(list.into_iter().map(MenuInfo::from).collect())
}

/// 获取菜单树（根据角色 ID 过滤）
pub async fn get_menu_tree(db: &DatabaseConnection, role_id: i64) -> Result<Vec<MenuInfo>> {
    let all_menus = if role_id == 888 {
        sys_menu::Entity::find()
            .order_by_asc(sys_menu::Column::Sort)
            .all(db)
            .await?
    } else {
        sys_menu::Entity::find()
            .filter(sys_menu::Column::Hidden.eq(false))
            .order_by_asc(sys_menu::Column::Sort)
            .all(db)
            .await?
    };

    let menus: Vec<MenuInfo> = all_menus.into_iter().map(MenuInfo::from).collect();
    Ok(build_menu_tree(menus, 0))
}

/// 将平铺菜单列表构建为树形结构
fn build_menu_tree(menus: Vec<MenuInfo>, parent_id: i32) -> Vec<MenuInfo> {
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
        keep_alive: Set(req.keep_alive),
        default_menu: Set(req.default_menu),
        title: Set(req.title),
        icon: Set(req.icon),
        close_tab: Set(req.close_tab),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 删除菜单
pub async fn delete_base_menu(db: &DatabaseConnection, id: i32) -> Result<()> {
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
pub async fn update_base_menu(db: &DatabaseConnection, id: i32, req: CreateMenuReq) -> Result<()> {
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
    active.keep_alive = Set(req.keep_alive);
    active.default_menu = Set(req.default_menu);
    active.title = Set(req.title);
    active.icon = Set(req.icon);
    active.close_tab = Set(req.close_tab);
    active.update(db).await?;
    Ok(())
}

/// 设置角色菜单关联
pub async fn add_menu_authority(
    db: &DatabaseConnection,
    menu_ids: Vec<i32>,
    role_id: i64,
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
pub async fn get_menu_authority(db: &DatabaseConnection, role_id: i64) -> Result<Vec<i32>> {
    let list = sys_authority_menu::Entity::find()
        .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(role_id))
        .all(db)
        .await?;
    Ok(list.into_iter().map(|m| m.sys_base_menu_id).collect())
}

/// 根据 ID 获取单个菜单
pub async fn get_base_menu_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<MenuInfo>> {
    let model = sys_menu::Entity::find_by_id(id).one(db).await?;
    Ok(model.map(MenuInfo::from))
}
