use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 角色数据权限关联表，对应 Gin-Vue-Admin 的 sys_data_authority_id
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_data_authority_id")]
pub struct Model {
    /// 角色ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_authority_authority_id: u64,
    /// 数据权限角色ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub data_authority_id_authority_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
