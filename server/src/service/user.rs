use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    model::system::{
        sys_user::{self, Entity as SysUserEntity},
        SysUser,
    },
    utils::{hash_password, verify_password},
};

/// 用户列表分页结果
pub struct UserListResult {
    pub list: Vec<SysUser>,
    pub total: u64,
    pub page: i64,
    pub page_size: i64,
}

/// 用户 Service，对应 Gin-Vue-Admin 的 userService
pub struct UserService;

impl UserService {
    /// 用户登录
    pub async fn login(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
    ) -> anyhow::Result<SysUser> {
        let user = SysUserEntity::find()
            .filter(sys_user::Column::Username.eq(username))
            .filter(sys_user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户名或密码错误"))?;

        if user.enable != 1 {
            return Err(anyhow::anyhow!("账号已被禁用，请联系管理员"));
        }

        if !verify_password(password, &user.password) {
            return Err(anyhow::anyhow!("用户名或密码错误"));
        }

        Ok(user)
    }

    /// 用户注册
    pub async fn register(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
        nick_name: &str,
        authority_id: i64,
    ) -> anyhow::Result<SysUser> {
        let exists = SysUserEntity::find()
            .filter(sys_user::Column::Username.eq(username))
            .filter(sys_user::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if exists.is_some() {
            return Err(anyhow::anyhow!("用户名已存在"));
        }

        let hashed = hash_password(password)?;

        let new_user = sys_user::ActiveModel {
            uuid: Set(Uuid::new_v4()),
            username: Set(username.to_string()),
            password: Set(hashed),
            nick_name: Set(nick_name.to_string()),
            header_img: Set(String::new()),
            phone: Set(String::new()),
            email: Set(String::new()),
            enable: Set(1),
            authority_id: Set(authority_id),
            ..Default::default()
        };

        let user = new_user.insert(db).await?;
        Ok(user)
    }

    /// 根据 ID 查询用户信息
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<SysUser> {
        SysUserEntity::find_by_id(id)
            .filter(sys_user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))
    }

    /// 修改用户密码（需要验证旧密码）
    pub async fn change_password(
        db: &DatabaseConnection,
        user_id: i64,
        old_password: &str,
        new_password: &str,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;

        if !verify_password(old_password, &user.password) {
            return Err(anyhow::anyhow!("原密码错误"));
        }

        let hashed = hash_password(new_password)?;
        let mut active: sys_user::ActiveModel = user.into();
        active.password = Set(hashed);
        active.update(db).await?;

        Ok(())
    }

    /// 获取用户列表（分页），对应 Gin-Vue-Admin 的 userService.GetUserInfoList()
    pub async fn get_user_list(
        db: &DatabaseConnection,
        page: i64,
        page_size: i64,
    ) -> anyhow::Result<UserListResult> {
        let page = page.max(1);
        let page_size = page_size.clamp(1, 100);

        let paginator = SysUserEntity::find()
            .filter(sys_user::Column::DeletedAt.is_null())
            .order_by_asc(sys_user::Column::Id)
            .paginate(db, page_size as u64);

        let total = paginator.num_items().await?;
        let list = paginator.fetch_page((page - 1) as u64).await?;

        Ok(UserListResult { list, total, page, page_size })
    }

    /// 设置用户角色，对应 Gin-Vue-Admin 的 userService.SetUserAuthority()
    pub async fn set_user_authority(
        db: &DatabaseConnection,
        user_id: i64,
        authority_id: i64,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;
        let mut active: sys_user::ActiveModel = user.into();
        active.authority_id = Set(authority_id);
        active.update(db).await?;
        Ok(())
    }

    /// 删除用户（软删除），对应 Gin-Vue-Admin 的 userService.DeleteUser()
    pub async fn delete_user(
        db: &DatabaseConnection,
        user_id: i64,
        operator_id: i64,
    ) -> anyhow::Result<()> {
        if user_id == operator_id {
            return Err(anyhow::anyhow!("不能删除自己"));
        }

        let user = Self::find_by_id(db, user_id).await?;
        let now = chrono::Local::now().naive_local();
        let mut active: sys_user::ActiveModel = user.into();
        active.deleted_at = Set(Some(now));
        active.update(db).await?;
        Ok(())
    }

    /// 修改用户个人信息，对应 Gin-Vue-Admin 的 userService.SetUserInfo()
    pub async fn update_user_info(
        db: &DatabaseConnection,
        user_id: i64,
        nick_name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        header_img: Option<String>,
    ) -> anyhow::Result<SysUser> {
        let user = Self::find_by_id(db, user_id).await?;
        let mut active: sys_user::ActiveModel = user.into();

        if let Some(v) = nick_name { active.nick_name = Set(v); }
        if let Some(v) = phone { active.phone = Set(v); }
        if let Some(v) = email { active.email = Set(v); }
        if let Some(v) = header_img { active.header_img = Set(v); }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    /// 重置用户密码（管理员操作，无需旧密码），对应 Gin-Vue-Admin 的 userService.ResetPassword()
    pub async fn reset_password(
        db: &DatabaseConnection,
        user_id: i64,
        new_password: &str,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;
        let hashed = hash_password(new_password)?;
        let mut active: sys_user::ActiveModel = user.into();
        active.password = Set(hashed);
        active.update(db).await?;
        Ok(())
    }

    /// 设置用户启用/禁用状态
    pub async fn set_user_enable(
        db: &DatabaseConnection,
        user_id: i64,
        enable: i8,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;
        let mut active: sys_user::ActiveModel = user.into();
        active.enable = Set(enable);
        active.update(db).await?;
        Ok(())
    }
}
