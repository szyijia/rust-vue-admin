use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统用户表，对应 Gin-Vue-Admin 的 SysUser
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
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
    pub enable: i8,
    /// 角色ID
    pub authority_id: i64,
    /// 创建时间
    pub created_at: Option<NaiveDateTime>,
    /// 更新时间
    pub updated_at: Option<NaiveDateTime>,
    /// 删除时间（软删除）
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
