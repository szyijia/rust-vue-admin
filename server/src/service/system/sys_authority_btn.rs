// 按钮权限服务
use anyhow::Result;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set, ActiveModelTrait, TransactionTrait,
};

use crate::model::system::sys_authority_btn;

/// 获取指定角色和菜单的按钮权限
pub async fn get_authority_btn(
    db: &DatabaseConnection,
    authority_id: u64,
    menu_id: u64,
) -> Result<Vec<u64>> {
    let records = sys_authority_btn::Entity::find()
        .filter(sys_authority_btn::Column::AuthorityId.eq(authority_id))
        .filter(sys_authority_btn::Column::SysMenuId.eq(menu_id))
        .all(db)
        .await?;
    Ok(records.into_iter().map(|r| r.sys_base_menu_btn_id).collect())
}

/// 设置角色按钮权限（全量替换）
pub async fn set_authority_btn(
    db: &DatabaseConnection,
    authority_id: u64,
    menu_id: u64,
    selected: Vec<u64>,
) -> Result<()> {
    let txn = db.begin().await?;

    // 删除旧的按钮权限
    sys_authority_btn::Entity::delete_many()
        .filter(sys_authority_btn::Column::AuthorityId.eq(authority_id))
        .filter(sys_authority_btn::Column::SysMenuId.eq(menu_id))
        .exec(&txn)
        .await?;

    // 插入新的按钮权限
    for btn_id in selected {
        let record = sys_authority_btn::ActiveModel {
            authority_id: Set(authority_id),
            sys_menu_id: Set(menu_id),
            sys_base_menu_btn_id: Set(btn_id),
            ..Default::default()
        };
        record.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(())
}

/// 判断用户是否有某按钮权限（可选功能）
pub async fn can_remove_authority_btn(
    db: &DatabaseConnection,
    authority_id: u64,
) -> Result<bool> {
    // 检查是否有该角色关联的按钮权限
    let count = sys_authority_btn::Entity::find()
        .filter(sys_authority_btn::Column::AuthorityId.eq(authority_id))
        .count(db)
        .await?;
    Ok(count == 0)
}
