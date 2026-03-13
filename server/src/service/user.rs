use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    model::system::{
        sys_user::{self, Entity as SysUserEntity},
        sys_user_authority,
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
        authority_id: u64,
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
    pub async fn find_by_id(db: &DatabaseConnection, id: u64) -> anyhow::Result<SysUser> {
        SysUserEntity::find_by_id(id)
            .filter(sys_user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))
    }

    /// 修改用户密码（需要验证旧密码）
    pub async fn change_password(
        db: &DatabaseConnection,
        user_id: u64,
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
        user_id: u64,
        authority_id: u64,
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
        user_id: u64,
        operator_id: u64,
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
        user_id: u64,
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
        user_id: u64,
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
        user_id: u64,
        enable: i64,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;
        let mut active: sys_user::ActiveModel = user.into();
        active.enable = Set(enable);
        active.update(db).await?;
        Ok(())
    }

    /// 设置用户的多角色权限（全量替换），对应 Gin-Vue-Admin 的 userService.SetUserAuthorities()
    pub async fn set_user_authorities(
        db: &DatabaseConnection,
        user_id: u64,
        authority_ids: Vec<u64>,
    ) -> anyhow::Result<()> {
        if authority_ids.is_empty() {
            return Err(anyhow::anyhow!("角色列表不能为空"));
        }

        // 确认用户存在
        let _user = Self::find_by_id(db, user_id).await?;

        let txn = db.begin().await?;

        // 删除该用户的所有旧角色关联
        sys_user_authority::Entity::delete_many()
            .filter(sys_user_authority::Column::SysUserId.eq(user_id))
            .exec(&txn)
            .await?;

        // 插入新的角色关联
        for authority_id in &authority_ids {
            let new_record = sys_user_authority::ActiveModel {
                sys_user_id: Set(user_id),
                sys_authority_authority_id: Set(*authority_id),
            };
            new_record.insert(&txn).await?;
        }

        // 将主角色设为第一个
        let mut active: sys_user::ActiveModel = _user.into();
        active.authority_id = Set(authority_ids[0]);
        active.update(&txn).await?;

        txn.commit().await?;
        Ok(())
    }

    /// 设置用户配置（JSON），对应 Gin-Vue-Admin 的 userService.SetSelfSetting()
    pub async fn set_self_setting(
        db: &DatabaseConnection,
        user_id: u64,
        setting: serde_json::Value,
    ) -> anyhow::Result<()> {
        let user = Self::find_by_id(db, user_id).await?;
        let mut active: sys_user::ActiveModel = user.into();
        active.origin_setting = Set(Some(setting.to_string()));
        active.update(db).await?;
        Ok(())
    }

    /// 获取用户的多角色列表
    pub async fn get_user_authorities(
        db: &DatabaseConnection,
        user_id: u64,
    ) -> anyhow::Result<Vec<u64>> {
        let records = sys_user_authority::Entity::find()
            .filter(sys_user_authority::Column::SysUserId.eq(user_id))
            .all(db)
            .await?;
        Ok(records.into_iter().map(|r| r.sys_authority_authority_id).collect())
    }
}
