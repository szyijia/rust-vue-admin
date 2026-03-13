use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::model::system::{
    sys_role,
    sys_data_authority_id,
    sys_user_authority,
    sys_user,
    sys_authority_menu,
    casbin_rule,
};

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityInfo {
    pub authority_id: u64,
    pub authority_name: String,
    pub parent_id: u64,
    pub default_router: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<AuthorityInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub data_authority_id: Vec<AuthorityIdItem>,
}

/// 数据权限角色ID项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityIdItem {
    pub authority_id: u64,
}

/// 创建角色
pub async fn create_authority(
    db: &DatabaseConnection,
    authority_id: u64,
    authority_name: String,
    parent_id: u64,
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
        data_authority_id: vec![],
    })
}

/// 删除角色
pub async fn delete_authority(db: &DatabaseConnection, authority_id: u64) -> Result<()> {
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
    authority_id: u64,
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
        data_authority_id: vec![],
    })
}

/// 获取角色列表
pub async fn get_authority_list(db: &DatabaseConnection) -> Result<Vec<AuthorityInfo>> {
    let list = sys_role::Entity::find()
        .order_by_asc(sys_role::Column::AuthorityId)
        .all(db)
        .await?;

    // 查询数据权限关联
    let data_auths = sys_data_authority_id::Entity::find().all(db).await.unwrap_or_default();
    let flat: Vec<AuthorityInfo> = list
        .into_iter()
        .map(|r| {
            let da_ids: Vec<AuthorityIdItem> = data_auths.iter()
                .filter(|da| da.sys_authority_authority_id == r.authority_id)
                .map(|da| AuthorityIdItem { authority_id: da.data_authority_id_authority_id })
                .collect();
            AuthorityInfo {
                authority_id: r.authority_id,
                authority_name: r.authority_name,
                parent_id: r.parent_id,
                default_router: r.default_router,
                children: vec![],
                data_authority_id: da_ids,
            }
        })
        .collect();

    Ok(build_authority_tree(flat, 0))
}

/// 将平铺角色列表构建为树形结构
fn build_authority_tree(authorities: Vec<AuthorityInfo>, parent_id: u64) -> Vec<AuthorityInfo> {
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
    authority_id: u64,
) -> Result<Option<AuthorityInfo>> {
    let model = sys_role::Entity::find_by_id(authority_id).one(db).await?;
    Ok(model.map(|r| AuthorityInfo {
        authority_id: r.authority_id,
        authority_name: r.authority_name,
        parent_id: r.parent_id,
        default_router: r.default_router,
        children: vec![],
        data_authority_id: vec![],
    }))
}

/// 复制角色，对应 Gin-Vue-Admin 的 authorityService.CopyAuthority()
pub async fn copy_authority(
    db: &DatabaseConnection,
    old_authority_id: u64,
    new_authority_id: u64,
    new_authority_name: String,
    new_parent_id: u64,
) -> Result<AuthorityInfo> {
    // 检查新角色ID是否已存在
    let existing = sys_role::Entity::find_by_id(new_authority_id).one(db).await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("角色 ID {} 已存在", new_authority_id));
    }

    let txn = db.begin().await?;

    // 创建新角色
    let new_role = sys_role::ActiveModel {
        authority_id: Set(new_authority_id),
        authority_name: Set(new_authority_name.clone()),
        parent_id: Set(new_parent_id),
        default_router: Set("dashboard".to_string()),
        ..Default::default()
    };
    new_role.insert(&txn).await?;

    // 复制菜单权限
    let old_menus = sys_authority_menu::Entity::find()
        .filter(sys_authority_menu::Column::SysAuthorityAuthorityId.eq(old_authority_id))
        .all(&txn)
        .await?;
    for menu in old_menus {
        let new_menu = sys_authority_menu::ActiveModel {
            sys_base_menu_id: Set(menu.sys_base_menu_id),
            sys_authority_authority_id: Set(new_authority_id),
        };
        new_menu.insert(&txn).await?;
    }

    // 复制 Casbin 权限策略
    let old_policies = casbin_rule::Entity::find()
        .filter(casbin_rule::Column::Ptype.eq("p"))
        .filter(casbin_rule::Column::V0.eq(old_authority_id.to_string()))
        .all(&txn)
        .await?;
    for policy in old_policies {
        let new_policy = casbin_rule::ActiveModel {
            ptype: Set("p".to_string()),
            v0: Set(new_authority_id.to_string()),
            v1: Set(policy.v1),
            v2: Set(policy.v2),
            v3: Set("".to_string()),
            v4: Set("".to_string()),
            v5: Set("".to_string()),
            ..Default::default()
        };
        new_policy.insert(&txn).await?;
    }

    txn.commit().await?;

    Ok(AuthorityInfo {
        authority_id: new_authority_id,
        authority_name: new_authority_name,
        parent_id: new_parent_id,
        default_router: "dashboard".to_string(),
        children: vec![],
        data_authority_id: vec![],
    })
}

/// 设置角色数据权限，对应 Gin-Vue-Admin 的 authorityService.SetDataAuthority()
pub async fn set_data_authority(
    db: &DatabaseConnection,
    authority_id: u64,
    data_authority_ids: Vec<u64>,
) -> Result<()> {
    let txn = db.begin().await?;

    // 删除旧的数据权限
    sys_data_authority_id::Entity::delete_many()
        .filter(sys_data_authority_id::Column::SysAuthorityAuthorityId.eq(authority_id))
        .exec(&txn)
        .await?;

    // 插入新的数据权限
    for data_id in data_authority_ids {
        let record = sys_data_authority_id::ActiveModel {
            sys_authority_authority_id: Set(authority_id),
            data_authority_id_authority_id: Set(data_id),
        };
        record.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(())
}

/// 获取拥有指定角色的所有用户ID，对应 Gin-Vue-Admin 的 authorityService.GetUserIdsByAuthorityId()
pub async fn get_user_ids_by_authority_id(
    db: &DatabaseConnection,
    authority_id: u64,
) -> Result<Vec<u64>> {
    let records = sys_user_authority::Entity::find()
        .filter(sys_user_authority::Column::SysAuthorityAuthorityId.eq(authority_id))
        .all(db)
        .await?;
    Ok(records.into_iter().map(|r| r.sys_user_id).collect())
}

/// 全量覆盖某角色关联的用户列表，对应 Gin-Vue-Admin 的 authorityService.SetRoleUsers()
pub async fn set_role_users(
    db: &DatabaseConnection,
    authority_id: u64,
    user_ids: Vec<u64>,
) -> Result<()> {
    let txn = db.begin().await?;

    // 查出当前拥有该角色的所有用户ID
    let existing_records = sys_user_authority::Entity::find()
        .filter(sys_user_authority::Column::SysAuthorityAuthorityId.eq(authority_id))
        .all(&txn)
        .await?;
    let current_user_ids: std::collections::HashSet<u64> = existing_records.iter().map(|r| r.sys_user_id).collect();
    let target_user_ids: std::collections::HashSet<u64> = user_ids.iter().copied().collect();

    // 删除该角色所有已有的用户关联
    sys_user_authority::Entity::delete_many()
        .filter(sys_user_authority::Column::SysAuthorityAuthorityId.eq(authority_id))
        .exec(&txn)
        .await?;

    // 对被移除的用户：若该角色是其主角色，则切换到其剩余角色
    for &uid in &current_user_ids {
        if target_user_ids.contains(&uid) {
            continue;
        }
        let user = sys_user::Entity::find_by_id(uid)
            .filter(sys_user::Column::DeletedAt.is_null())
            .one(&txn)
            .await?;
        if let Some(user) = user {
            if user.authority_id == authority_id {
                // 查找该用户的另一个角色
                let another = sys_user_authority::Entity::find()
                    .filter(sys_user_authority::Column::SysUserId.eq(uid))
                    .one(&txn)
                    .await?;
                if let Some(another) = another {
                    let mut active: sys_user::ActiveModel = user.into();
                    active.authority_id = Set(another.sys_authority_authority_id);
                    active.update(&txn).await?;
                }
            }
        }
    }

    // 批量插入新的关联记录
    for uid in &user_ids {
        let record = sys_user_authority::ActiveModel {
            sys_user_id: Set(*uid),
            sys_authority_authority_id: Set(authority_id),
        };
        record.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(())
}
