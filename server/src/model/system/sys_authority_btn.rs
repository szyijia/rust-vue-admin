use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 角色按钮权限关联表，对应 Gin-Vue-Admin 的 SysAuthorityBtn
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_authority_btns")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// 角色ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub authority_id: u64,
    /// 菜单ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_menu_id: u64,
    /// 按钮ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_base_menu_btn_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
