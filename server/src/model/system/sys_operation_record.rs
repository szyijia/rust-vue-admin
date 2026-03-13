use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 操作记录表，对应 Gin-Vue-Admin 的 SysOperationRecord
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_operation_records")]
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
    /// 请求IP
    pub ip: String,
    /// 请求方法
    pub method: String,
    /// 请求路径
    pub path: String,
    /// 请求状态码
    pub status: i64,
    /// 延迟（纳秒）
    pub latency: i64,
    /// 代理
    #[sea_orm(column_type = "Text")]
    pub agent: String,
    /// 错误信息
    pub error_message: String,
    /// 请求Body
    #[sea_orm(column_type = "Text")]
    pub body: String,
    /// 响应Body
    #[sea_orm(column_type = "Text")]
    pub resp: String,
    /// 用户ID
    pub user_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
