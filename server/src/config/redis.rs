use serde::Deserialize;

/// Redis 配置，对应 config.yaml redis 节
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    /// 是否使用集群模式
    pub use_cluster: bool,
    /// 单机地址，如 "127.0.0.1:6379"
    pub addr: String,
    /// 密码
    pub password: String,
    /// 数据库编号
    pub db: i64,
    /// 集群地址列表
    pub cluster_addrs: Vec<String>,
}
