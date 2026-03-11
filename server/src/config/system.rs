use serde::Deserialize;

/// 系统配置，对应 config.yaml system 节
#[derive(Debug, Deserialize, Clone)]
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
    /// 是否禁用自动数据库迁移
    pub disable_auto_migrate: bool,
}
