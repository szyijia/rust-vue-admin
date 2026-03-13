// 允许未使用的代码警告（后续阶段会逐步使用）
#![allow(dead_code)]

mod config;
mod core;
mod global;
mod initialize;
mod migration;
mod router;

// 以下模块暂时声明，后续阶段逐步实现
mod api;
mod middleware;
mod model;
mod service;
mod utils;

use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    // 1. 加载配置（对应 Gin-Vue-Admin: core.Viper()）
    let app_config = match initialize::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ 配置加载失败: {}", e);
            std::process::exit(1);
        }
    };

    // 2. 初始化日志（对应 Gin-Vue-Admin: core.Zap()）
    core::init_logger(&app_config.log);

    // 3. 构建全局状态（初始无 DB/Redis）
    let mut state = global::AppState::new(app_config.clone());

    // 4. 尝试连接数据库（如果配置了的话）
    //    对应 Gin-Vue-Admin: 若 GVA_DB != nil 则跳过初始化页面
    //    若数据库配置为空（db_name 为空），则跳过，等待用户通过 /init/initdb 接口初始化
    let db_configured = match app_config.system.db_type.to_lowercase().as_str() {
        "mysql" => !app_config.mysql.db_name.is_empty() && !app_config.mysql.path.is_empty(),
        "pgsql" | "postgres" | "postgresql" => !app_config.pgsql.db_name.is_empty() && !app_config.pgsql.path.is_empty(),
        "sqlite" => !app_config.sqlite.path.is_empty(),
        _ => false,
    };

    if db_configured {
        match initialize::init_db(&app_config).await {
            Ok(db) => {
                // 自动执行数据库迁移（仅增量模式，不删表）
                {
                    use sea_orm_migration::MigratorTrait;

                    // 迁移前：输出当前迁移状态报告
                    match migration::Migrator::get_migration_with_status(&db).await {
                        Ok(migrations) => {
                            info!("📋 数据库迁移状态报告：");
                            for m in &migrations {
                                let status_icon = match m.status() {
                                    sea_orm_migration::MigrationStatus::Applied => "✅ 已执行",
                                    sea_orm_migration::MigrationStatus::Pending => "⏳ 待执行",
                                };
                                info!("  {} - {}", status_icon, m.name());
                            }
                            let pending_count = migrations.iter()
                                .filter(|m| m.status() == sea_orm_migration::MigrationStatus::Pending)
                                .count();
                            if pending_count == 0 {
                                info!("✅ 所有迁移均已执行，无需操作");
                            } else {
                                info!("📌 共 {} 个待执行迁移，开始执行...", pending_count);
                            }
                        }
                        Err(e) => warn!("⚠️  获取迁移状态失败: {}，将尝试直接执行迁移", e),
                    }

                    // 执行增量迁移（只执行未运行过的迁移，表已存在会被 IF NOT EXISTS 跳过）
                    match migration::Migrator::up(&db, None).await {
                        Ok(_) => info!("✅ 数据库迁移完成"),
                        Err(e) => warn!("⚠️  数据库迁移失败: {}（已存在的表不受影响）", e),
                    }
                }

                // 初始化 Casbin
                match initialize::init_casbin(&db).await {
                    Ok(enforcer) => {
                        state.set_enforcer(enforcer).await;
                        info!("✅ Casbin 权限引擎初始化完成");
                    }
                    Err(e) => warn!("⚠️  Casbin 初始化失败: {}", e),
                }

                state.set_db(db).await;
                info!("✅ 数据库连接成功");
            }
            Err(e) => {
                warn!("⚠️  数据库连接失败: {}，服务将在无数据库模式下启动（请通过 /init/initdb 初始化）", e);
            }
        }
    } else {
        info!("ℹ️  数据库未配置，服务以初始化模式启动，请访问 /init/checkdb 进行初始化");
    }

    // 5. 初始化 Redis（对应 Gin-Vue-Admin: initialize.Redis()）
    if app_config.system.use_redis {
        match initialize::init_redis(&app_config.redis).await {
            Ok(redis_conn) => {
                state = state.with_redis(redis_conn);
            }
            Err(e) => {
                warn!("⚠️  Redis 连接失败: {}，服务将在无 Redis 模式下启动", e);
            }
        }
    }

    // 6. 启动配置文件热重载监听（对应 Gin-Vue-Admin: viper.WatchConfig()）
    initialize::start_config_watcher(state.clone());

    // 7. 初始化路由（对应 Gin-Vue-Admin: initialize.Routers()）
    let router = router::init_router(state);

    // 8. 启动服务器（对应 Gin-Vue-Admin: core.RunServer()）
    if let Err(e) = core::run_server(router, &app_config.system).await {
        error!("服务器运行错误: {}", e);
        std::process::exit(1);
    }
}


