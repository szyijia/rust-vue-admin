use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 错误日志表，对应 Gin-Vue-Admin 的 SysError
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_error")]
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
    /// 错误来源
    #[sea_orm(column_type = "Text", nullable)]
    pub form: Option<String>,
    /// 错误内容
    #[sea_orm(column_type = "Text", nullable)]
    pub info: Option<String>,
    /// 日志等级
    pub level: Option<String>,
    /// 解决方案
    #[sea_orm(column_type = "Text", nullable)]
    pub solution: Option<String>,
    /// 处理状态：未处理/处理中/处理完成
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
