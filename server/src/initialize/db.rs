use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::info;

use crate::config::AppConfig;

/// 初始化数据库连接，对应 Gin-Vue-Admin 的 initialize.Gorm()
///
/// 根据 system.db_type 自动选择数据库类型：
/// - mysql    → MySQL / MariaDB
/// - postgres → PostgreSQL
/// - sqlite   → SQLite（开发/测试用）
pub async fn init_db(cfg: &AppConfig) -> anyhow::Result<DatabaseConnection> {
    let db_type = cfg.system.db_type.to_lowercase();

    let (dsn, max_idle, max_open, log_mode) = match db_type.as_str() {
        "mysql" => {
            let c = &cfg.mysql;
            if c.path.is_empty() || c.db_name.is_empty() {
                anyhow::bail!(
                    "MySQL 配置不完整，请检查 config.yaml 中的 mysql.path / mysql.db_name"
                );
            }
            (c.dsn(), c.max_idle_conns, c.max_open_conns, c.log_mode.clone())
        }
        "postgres" | "postgresql" | "pgsql" => {
            let c = &cfg.pgsql;
            if c.path.is_empty() || c.db_name.is_empty() {
                anyhow::bail!(
                    "PostgreSQL 配置不完整，请检查 config.yaml 中的 pgsql.path / pgsql.db_name"
                );
            }
            (c.dsn(), c.max_idle_conns, c.max_open_conns, c.log_mode.clone())
        }
        "sqlite" => {
            let c = &cfg.sqlite;
            let dsn = c.dsn();
            info!(dsn = %dsn, "SQLite DSN");
            (dsn, c.max_idle_conns, c.max_open_conns, c.log_mode.clone())
        }
        other => {
            anyhow::bail!("不支持的数据库类型: {}，请使用 mysql/postgres/sqlite", other);
        }
    };

    info!(db_type = %db_type, "正在连接数据库...");

    // 构建连接选项
    let mut opt = ConnectOptions::new(dsn);
    opt.max_connections(max_open)
        .min_connections(max_idle)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(is_sql_logging_enabled(&log_mode));

    let db = Database::connect(opt).await?;

    // 验证连接
    db.ping().await?;

    info!(db_type = %db_type, "✅ 数据库连接成功");

    Ok(db)
}

/// 根据日志模式决定是否开启 SQL 日志
fn is_sql_logging_enabled(log_mode: &str) -> bool {
    matches!(log_mode.to_lowercase().as_str(), "info" | "warn")
}
