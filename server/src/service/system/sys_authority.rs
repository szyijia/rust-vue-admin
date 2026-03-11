use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::sys_role;

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityInfo {
    pub authority_id: i64,
    pub authority_name: String,
    pub parent_id: i64,
    pub default_router: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<AuthorityInfo>,
}

/// 创建角色
pub async fn create_authority(
    db: &DatabaseConnection,
    authority_id: i64,
    authority_name: String,
    parent_id: i64,
    default_router: String,
) -> Result<AuthorityInfo> {
    // 检查是否已存在
    let existing = sys_role::Entity::find_by_id(authority_id).one(db).await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("角色 ID {} 已存在", authority_id));
    }

    let model = sys_role::ActiveModel {
        authority_id: Set(authority_id),
        authority_name: Set(authority_name.clone()),
        parent_id: Set(parent_id),
        default_router: Set(default_router.clone()),
        ..Default::default()
    };
    model.insert(db).await?;

    Ok(AuthorityInfo {
        authority_id,
        authority_name,
        parent_id,
        default_router,
        children: vec![],
    })
}

/// 删除角色
pub async fn delete_authority(db: &DatabaseConnection, authority_id: i64) -> Result<()> {
    if authority_id == 888 {
        return Err(anyhow::anyhow!("超级管理员角色不可删除"));
    }

    // 检查是否有子角色
    let children = sys_role::Entity::find()
        .filter(sys_role::Column::ParentId.eq(authority_id))
        .one(db)
        .await?;
    if children.is_some() {
        return Err(anyhow::anyhow!("该角色下存在子角色，请先删除子角色"));
    }

    sys_role::Entity::delete_by_id(authority_id).exec(db).await?;
    Ok(())
}

/// 更新角色
pub async fn update_authority(
    db: &DatabaseConnection,
    authority_id: i64,
    authority_name: String,
    default_router: String,
) -> Result<AuthorityInfo> {
    let model = sys_role::Entity::find_by_id(authority_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("角色不存在"))?;

    let parent_id = model.parent_id;
    let mut active: sys_role::ActiveModel = model.into();
    active.authority_name = Set(authority_name.clone());
    active.default_router = Set(default_router.clone());
    active.update(db).await?;

    Ok(AuthorityInfo {
        authority_id,
        authority_name,
        parent_id,
        default_router,
        children: vec![],
    })
}

/// 获取角色列表
pub async fn get_authority_list(db: &DatabaseConnection) -> Result<Vec<AuthorityInfo>> {
    let list = sys_role::Entity::find()
        .order_by_asc(sys_role::Column::AuthorityId)
        .all(db)
        .await?;

    let flat: Vec<AuthorityInfo> = list
        .into_iter()
        .map(|r| AuthorityInfo {
            authority_id: r.authority_id,
            authority_name: r.authority_name,
            parent_id: r.parent_id,
            default_router: r.default_router,
            children: vec![],
        })
        .collect();

    Ok(build_authority_tree(flat, 0))
}

/// 将平铺角色列表构建为树形结构
fn build_authority_tree(authorities: Vec<AuthorityInfo>, parent_id: i64) -> Vec<AuthorityInfo> {
    authorities
        .iter()
        .filter(|a| a.parent_id == parent_id)
        .map(|a| {
            let mut node = a.clone();
            node.children = build_authority_tree(authorities.clone(), a.authority_id);
            node
        })
        .collect()
}

/// 根据 ID 获取角色
pub async fn get_authority_by_id(
    db: &DatabaseConnection,
    authority_id: i64,
) -> Result<Option<AuthorityInfo>> {
    let model = sys_role::Entity::find_by_id(authority_id).one(db).await?;
    Ok(model.map(|r| AuthorityInfo {
        authority_id: r.authority_id,
        authority_name: r.authority_name,
        parent_id: r.parent_id,
        default_router: r.default_router,
        children: vec![],
    }))
}
