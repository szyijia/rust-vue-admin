use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 用户-角色多对多关联表，对应 Gin-Vue-Admin 的 SysUserAuthority
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user_authority")]
pub struct Model {
    /// 用户ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_user_id: u64,
    /// 角色ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_authority_authority_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
