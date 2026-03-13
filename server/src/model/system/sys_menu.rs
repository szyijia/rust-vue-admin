use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 基础菜单表（sys_base_menus）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_base_menus")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<DateTimeUtc>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<DateTimeUtc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTimeUtc>,
    pub menu_level: Option<u64>,
    pub parent_id: u64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i64,
    pub active_name: Option<String>,
    pub keep_alive: bool,
    pub default_menu: bool,
    pub title: String,
    pub icon: String,
    pub close_tab: bool,
    pub transition_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
