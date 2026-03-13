use serde::Deserialize;
use tracing::warn;

/// 系统配置，对应 config.yaml system 节
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SystemConfig {
    /// 运行环境: local/development/production
    pub env: String,
    /// 服务端口
    pub addr: u16,
    /// 数据库类型: mysql/postgres/sqlite
    pub db_type: String,
    /// OSS 类型: local/minio/aliyun/tencent/aws
    pub oss_type: String,
    /// 是否使用 Redis
    pub use_redis: bool,
    /// 是否开启多点登录限制
    pub use_multipoint: bool,
    /// IP 限流次数（每小时）
    pub ip_limit_count: u32,
    /// IP 限流时间窗口（秒）
    pub ip_limit_time: u32,
    /// 路由全局前缀
    pub router_prefix: String,
    /// 是否开启严格角色模式
    pub use_strict_auth: bool,
}

impl SystemConfig {
    /// 不可热重载的字段说明（启动时打印）
    pub const SKIP_REASONS: &[&str] = &[
        "system.addr    — 服务监听端口，变更需重启服务重新绑定端口",
        "system.db_type — 数据库类型，变更需重启服务",
    ];

    /// 合并配置：不可热重载的字段保留旧值（self），其余用新值
    ///
    /// 不可热重载的字段：addr, db_type
    /// 其余字段（env, oss_type, use_redis, ...）全部自动使用新值
    pub fn merge_from_new(&self, new: &SystemConfig) -> SystemConfig {
        SystemConfig {
            // ❌ 不可热重载：保留旧值
            addr: self.addr,
            db_type: self.db_type.clone(),
            // ✅ 其余字段全部使用新值（新增字段默认可热重载，无需改此处）
            ..new.clone()
        }
    }

    /// 检测不可热重载的字段是否变更，输出日志提示
    pub fn log_skipped_changes(&self, new: &SystemConfig) {
        if self.addr != new.addr {
            warn!(
                "⚠️  [配置热重载] system.addr 端口已变更 ({} → {})，但需要重启服务才能生效",
                self.addr, new.addr
            );
        }
        if self.db_type != new.db_type {
            warn!(
                "⚠️  [配置热重载] system.db_type 已变更 ({} → {})，但需要重启服务才能生效",
                self.db_type, new.db_type
            );
        }
    }
}
