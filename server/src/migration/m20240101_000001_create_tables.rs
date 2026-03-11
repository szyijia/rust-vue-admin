use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ========== 1. sys_authorities ==========
        manager
            .create_table(
                Table::create()
                    .table(SysAuthorities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysAuthorities::AuthorityId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SysAuthorities::AuthorityName)
                            .string_len(30)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysAuthorities::ParentId)
                            .big_integer()
                            .not_null()
                            .default(0i64),
                    )
                    .col(
                        ColumnDef::new(SysAuthorities::DefaultRouter)
                            .string_len(191)
                            .not_null()
                            .default("dashboard"),
                    )
                    .col(ColumnDef::new(SysAuthorities::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysAuthorities::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysAuthorities::DeletedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        // ========== 2. sys_users ==========
        manager
            .create_table(
                Table::create()
                    .table(SysUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Uuid)
                            .char_len(36)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Username)
                            .string_len(191)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Password)
                            .string_len(191)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUsers::NickName)
                            .string_len(191)
                            .not_null()
                            .default("系统用户"),
                    )
                    .col(
                        ColumnDef::new(SysUsers::HeaderImg)
                            .string_len(191)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Phone)
                            .string_len(191)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Email)
                            .string_len(191)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUsers::Enable)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(SysUsers::AuthorityId)
                            .big_integer()
                            .not_null()
                            .default(888i64),
                    )
                    .col(ColumnDef::new(SysUsers::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysUsers::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysUsers::DeletedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        // ========== 3. jwt_blacklists ==========
        manager
            .create_table(
                Table::create()
                    .table(JwtBlacklists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JwtBlacklists::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JwtBlacklists::Jwt).text().not_null())
                    .col(ColumnDef::new(JwtBlacklists::CreatedAt).date_time().null())
                    .col(ColumnDef::new(JwtBlacklists::UpdatedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        // ========== 4. sys_base_menus ==========
        manager
            .create_table(
                Table::create()
                    .table(SysBaseMenus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysBaseMenus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysBaseMenus::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysBaseMenus::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysBaseMenus::DeletedAt).date_time().null())
                    .col(ColumnDef::new(SysBaseMenus::ParentId).integer().not_null().default(0i32))
                    .col(ColumnDef::new(SysBaseMenus::Path).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysBaseMenus::Name).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysBaseMenus::Hidden).boolean().not_null().default(false))
                    .col(ColumnDef::new(SysBaseMenus::Component).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysBaseMenus::Sort).integer().not_null().default(0))
                    .col(ColumnDef::new(SysBaseMenus::KeepAlive).boolean().not_null().default(false))
                    .col(ColumnDef::new(SysBaseMenus::DefaultMenu).boolean().not_null().default(false))
                    .col(ColumnDef::new(SysBaseMenus::Title).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysBaseMenus::Icon).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysBaseMenus::CloseTab).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        // ========== 5. sys_authority_menus（角色-菜单关联表） ==========
        manager
            .create_table(
                Table::create()
                    .table(SysAuthorityMenus::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SysAuthorityMenus::SysBaseMenuId).integer().not_null())
                    .col(
                        ColumnDef::new(SysAuthorityMenus::SysAuthorityAuthorityId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(SysAuthorityMenus::SysBaseMenuId)
                            .col(SysAuthorityMenus::SysAuthorityAuthorityId),
                    )
                    .to_owned(),
            )
            .await?;

        // ========== 6. sys_apis ==========
        manager
            .create_table(
                Table::create()
                    .table(SysApis::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysApis::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysApis::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysApis::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysApis::DeletedAt).date_time().null())
                    .col(ColumnDef::new(SysApis::Path).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysApis::Description).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysApis::ApiGroup).string_len(191).not_null().default(""))
                    .col(ColumnDef::new(SysApis::Method).string_len(20).not_null().default("POST"))
                    .to_owned(),
            )
            .await?;

        // ========== 7. casbin_rules ==========
        manager
            .create_table(
                Table::create()
                    .table(CasbinRules::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CasbinRules::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CasbinRules::Ptype).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V0).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V1).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V2).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V3).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V4).string_len(100).not_null().default(""))
                    .col(ColumnDef::new(CasbinRules::V5).string_len(100).not_null().default(""))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 按外键依赖逆序删除
        manager.drop_table(Table::drop().table(CasbinRules::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(SysApis::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(SysAuthorityMenus::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(SysBaseMenus::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(JwtBlacklists::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(SysUsers::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(SysAuthorities::Table).to_owned()).await?;
        Ok(())
    }
}

// ========== 表名枚举定义 ==========

#[derive(Iden)]
enum SysAuthorities {
    Table,
    AuthorityId,
    AuthorityName,
    ParentId,
    DefaultRouter,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum SysUsers {
    Table,
    Id,
    Uuid,
    Username,
    Password,
    NickName,
    HeaderImg,
    Phone,
    Email,
    Enable,
    AuthorityId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum JwtBlacklists {
    Table,
    Id,
    Jwt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum SysBaseMenus {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ParentId,
    Path,
    Name,
    Hidden,
    Component,
    Sort,
    KeepAlive,
    DefaultMenu,
    Title,
    Icon,
    CloseTab,
}

#[derive(Iden)]
enum SysAuthorityMenus {
    Table,
    SysBaseMenuId,
    SysAuthorityAuthorityId,
}

#[derive(Iden)]
enum SysApis {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Path,
    Description,
    ApiGroup,
    Method,
}

#[derive(Iden)]
enum CasbinRules {
    Table,
    Id,
    Ptype,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
}
