use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// JWT 黑名单表，对应 Gin-Vue-Admin 的 JwtBlacklist
/// 用于存储已注销的 JWT Token，防止 Token 被重复使用
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "jwt_blacklists")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// JWT Token 字符串
    #[sea_orm(column_type = "Text")]
    pub jwt: String,
    /// 创建时间
    pub created_at: Option<NaiveDateTime>,
    /// 更新时间
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
