use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统用户表，对应 Gin-Vue-Admin 的 SysUser
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "ID")]
    pub id: u64,
    /// 用户UUID（唯一标识）
    pub uuid: Uuid,
    /// 用户名（登录名）
    #[sea_orm(unique)]
    pub username: String,
    /// 密码（bcrypt 哈希）
    pub password: String,
    /// 昵称
    pub nick_name: String,
    /// 头像
    pub header_img: String,
    /// 手机号
    pub phone: String,
    /// 邮箱
    pub email: String,
    /// 是否启用：0=禁用，1=启用
    pub enable: i64,
    /// 角色ID
    pub authority_id: u64,
    /// 用户配置（JSON格式）
    #[sea_orm(column_type = "Text", nullable)]
    pub origin_setting: Option<String>,
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
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::AuthorityId",
        to = "super::sys_role::Column::AuthorityId"
    )]
    Role,
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
