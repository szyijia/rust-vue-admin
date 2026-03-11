use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统 API 表（sys_apis）
/// 对应 Gin-Vue-Admin 的 SysApi
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_apis")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
    pub deleted_at: Option<DateTimeUtc>,
    /// API 路径，如 /user/getUserInfo
    pub path: String,
    /// API 描述
    pub description: String,
    /// API 分组
    pub api_group: String,
    /// 请求方法：GET/POST/PUT/DELETE
    pub method: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
