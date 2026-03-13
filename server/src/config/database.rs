use serde::Deserialize;

/// 数据库通用配置
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MysqlConfig {
    pub path: String,
    pub port: String,
    pub config: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub log_mode: String,
}

impl MysqlConfig {
    /// 生成 DSN 连接字符串（sea-orm/sqlx 格式）
    /// 格式：mysql://user:password@host:port/dbname?charset=utf8mb4
    pub fn dsn(&self) -> String {
        // 过滤掉 Go 特有的参数（parseTime、loc），只保留 sea-orm 支持的参数
        let params: Vec<&str> = self
            .config
            .split('&')
            .filter(|p| !p.starts_with("parseTime") && !p.starts_with("loc="))
            .collect();
        let query = params.join("&");

        if query.is_empty() {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.path, self.port, self.db_name
            )
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}?{}",
                self.username, self.password, self.path, self.port, self.db_name, query
            )
        }
    }
}

/// PostgreSQL 配置
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PgsqlConfig {
    pub path: String,
    pub port: String,
    pub config: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub log_mode: String,
}

impl PgsqlConfig {
    /// 生成 DSN 连接字符串
    pub fn dsn(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?{}",
            self.username, self.password, self.path, self.port, self.db_name, self.config
        )
    }
}

/// SQLite 配置
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SqliteConfig {
    pub path: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub log_mode: String,
}

impl SqliteConfig {
    /// 生成 DSN 连接字符串
    pub fn dsn(&self) -> String {
        // SQLite DSN 格式（sea-orm/sqlx）：
        // - 内存数据库: sqlite::memory:
        // - 绝对路径: sqlite:///absolute/path/foo.db?mode=rwc
        // - 相对路径: sqlite:./foo.db?mode=rwc
        // 注意：必须加 ?mode=rwc 才能自动创建文件
        if self.path.is_empty() {
            "sqlite::memory:".to_string()
        } else if self.path.starts_with('/') {
            // 绝对路径
            format!("sqlite://{}?mode=rwc", self.path)
        } else if self.path.starts_with("./") || self.path.starts_with("../") {
            // 已有相对路径前缀
            format!("sqlite:{}?mode=rwc", self.path)
        } else {
            // 相对路径，加 ./ 前缀
            format!("sqlite:./{}?mode=rwc", self.path)
        }
    }
}
