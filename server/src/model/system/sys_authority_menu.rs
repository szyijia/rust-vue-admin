use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 角色菜单关联表（sys_authority_menus）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_authority_menus")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_base_menu_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_authority_authority_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
