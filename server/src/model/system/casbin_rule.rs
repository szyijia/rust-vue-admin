use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Casbin 规则表（casbin_rule）
/// 对应 Gin-Vue-Admin 的 CasbinRule
/// 格式：ptype="p", v0=角色ID, v1=路径, v2=方法
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "casbin_rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "ID")]
    pub id: u64,
    /// 策略类型：p（权限策略）或 g（角色继承）
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
