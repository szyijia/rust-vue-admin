use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统角色表，对应 Gin-Vue-Admin 的 SysAuthority
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_authorities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub authority_id: u64,
    /// 角色名称
    pub authority_name: String,
    /// 父角色ID（0 表示顶级）
    pub parent_id: u64,
    /// 默认路由（登录后跳转）
    pub default_router: String,
    /// 创建时间
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<NaiveDateTime>,
    /// 更新时间
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    /// 删除时间（软删除）
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_user::Entity")]
    Users,
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
