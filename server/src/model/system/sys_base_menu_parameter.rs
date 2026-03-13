use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 菜单参数表，对应 Gin-Vue-Admin 的 SysBaseMenuParameter
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_base_menu_parameters")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    /// 关联的菜单ID
    pub sys_base_menu_id: u64,
    /// 地址栏携带参数为 params 还是 query
    #[sea_orm(column_name = "type")]
    pub r#type: Option<String>,
    /// 地址栏携带参数的 key
    pub key: Option<String>,
    /// 地址栏携带参数的值
    pub value: Option<String>,
    /// 创建时间
    pub created_at: Option<NaiveDateTime>,
    /// 更新时间
    pub updated_at: Option<NaiveDateTime>,
    /// 删除时间（软删除）
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
