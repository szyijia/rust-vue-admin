use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 参数管理表，对应 Gin-Vue-Admin 的 SysParams
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_params")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
    /// 参数名称
    pub name: String,
    /// 参数键
    #[sea_orm(column_name = "key")]
    pub key: String,
    /// 参数值
    pub value: String,
    /// 参数说明
    pub desc: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
