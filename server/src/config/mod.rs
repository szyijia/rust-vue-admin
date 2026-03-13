pub mod captcha;
pub mod cors;
pub mod database;
pub mod email;
pub mod jwt;
pub mod log;
pub mod redis;
pub mod system;

pub use captcha::CaptchaConfig;
pub use cors::CorsConfig;
pub use database::{MysqlConfig, PgsqlConfig, SqliteConfig};
pub use email::EmailConfig;
pub use jwt::JwtConfig;
pub use log::LogConfig;
pub use redis::RedisConfig;
pub use system::SystemConfig;

use serde::Deserialize;
use tracing::{info, warn};

/// 全局配置结构体，对应 config.yaml 顶层结构
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AppConfig {
    pub jwt: JwtConfig,
    pub log: LogConfig,
    pub redis: RedisConfig,
    pub email: EmailConfig,
    pub system: SystemConfig,
    pub captcha: CaptchaConfig,
    pub mysql: MysqlConfig,
    pub pgsql: PgsqlConfig,
    pub sqlite: SqliteConfig,
    pub cors: CorsConfig,
}

impl AppConfig {
    /// 不可热重载的顶层配置项说明（启动时打印）
    ///
    /// 新增的顶层配置默认可热重载，不需要改这里；
    /// 只有需要重启才能生效的配置才需要加到这个列表和下面的方法中。
    pub const SKIP_REASONS: &[&str] = &[
        "mysql.*      — 数据库连接配置，变更需重启服务重新建立连接池",
        "pgsql.*      — 数据库连接配置，变更需重启服务重新建立连接池",
        "sqlite.*     — 数据库连接配置，变更需重启服务重新建立连接池",
        "redis.*      — Redis 连接配置，变更需重启服务重新建立连接",
        "log.*        — 日志配置，tracing 订阅器在启动时初始化，变更需重启服务",
    ];

    /// 合并配置：不可热重载的字段保留旧值（self），其余用新值
    ///
    /// 设计原则：**新增的顶层配置字段默认使用新值（可热重载）**，
    /// 只有明确标记为不可热重载的字段才保留旧值。
    /// 这样新增配置时不需要修改此方法。
    pub fn merge_from_new(&self, new: &AppConfig) -> AppConfig {
        AppConfig {
            // ❌ 不可热重载：保留旧值
            log: self.log.clone(),
            mysql: self.mysql.clone(),
            pgsql: self.pgsql.clone(),
            sqlite: self.sqlite.clone(),
            redis: self.redis.clone(),

            // ⚠️ system: 混合字段，委托给 SystemConfig 自己处理
            system: self.system.merge_from_new(&new.system),

            // ✅ 其余全部使用新值（新增顶层字段默认可热重载，无需改此处）
            ..new.clone()
        }
    }

    /// 检测不可热重载的配置项是否变更，并输出日志提示需要重启
    ///
    /// 同时也记录已热重载的变更信息。
    /// 新增可热重载的配置字段无需修改此方法。
    pub fn log_skipped_changes(&self, new: &AppConfig) {
        // —— 不可热重载的配置变更警告 ——
        if self.mysql != new.mysql {
            warn!("⚠️  [配置热重载] mysql 配置已变更，但需要重启服务才能生效");
        }
        if self.pgsql != new.pgsql {
            warn!("⚠️  [配置热重载] pgsql 配置已变更，但需要重启服务才能生效");
        }
        if self.sqlite != new.sqlite {
            warn!("⚠️  [配置热重载] sqlite 配置已变更，但需要重启服务才能生效");
        }
        if self.redis != new.redis {
            warn!("⚠️  [配置热重载] redis 配置已变更，但需要重启服务才能生效");
        }
        if self.log != new.log {
            warn!("⚠️  [配置热重载] log 配置已变更，但需要重启服务才能生效");
        }
        // system 中不可热重载的字段
        self.system.log_skipped_changes(&new.system);

        // —— 已热重载的变更信息 ——
        if self.jwt != new.jwt {
            info!("🔄 [配置热重载] jwt 配置已更新");
        }
        if self.captcha != new.captcha {
            info!("🔄 [配置热重载] captcha 配置已更新");
        }
        if self.cors != new.cors {
            info!("🔄 [配置热重载] cors 配置已更新（注意：CORS layer 在启动时构建，部分变更可能需要重启）");
        }
        if self.email != new.email {
            info!("🔄 [配置热重载] email 配置已更新");
        }
        // system 中可热重载的字段
        if self.system.ip_limit_count != new.system.ip_limit_count
            || self.system.ip_limit_time != new.system.ip_limit_time
        {
            info!(
                "🔄 [配置热重载] IP 限流配置已更新 (count: {} → {}, time: {}s → {}s)",
                self.system.ip_limit_count,
                new.system.ip_limit_count,
                self.system.ip_limit_time,
                new.system.ip_limit_time
            );
        }
        if self.system.use_redis != new.system.use_redis {
            info!(
                "🔄 [配置热重载] use_redis 已更新: {} → {}",
                self.system.use_redis, new.system.use_redis
            );
        }
        if self.system.use_strict_auth != new.system.use_strict_auth {
            info!(
                "🔄 [配置热重载] use_strict_auth 已更新: {} → {}",
                self.system.use_strict_auth, new.system.use_strict_auth
            );
        }
    }
}
