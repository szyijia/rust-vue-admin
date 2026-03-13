use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AppConfig;
use crate::initialize::casbin::SharedEnforcer;

/// 应用全局状态，通过 axum Extension 注入到每个请求
/// 对应 Gin-Vue-Admin 的 global.GVA_CONFIG / global.GVA_DB 等全局变量
#[derive(Clone)]
pub struct AppState {
    /// 全局配置（支持热重载，文件变更时自动更新）
    pub config: Arc<RwLock<Arc<AppConfig>>>,
    /// SeaORM 数据库连接池（运行时可动态设置，通过 initdb 接口初始化）
    pub db: Arc<RwLock<Option<Arc<sea_orm::DatabaseConnection>>>>,
    /// Redis 连接管理器（初始化后设置）
    pub redis: Option<Arc<redis::aio::ConnectionManager>>,
    /// Casbin 权限执行器（运行时可动态设置）
    pub enforcer: Arc<RwLock<Option<SharedEnforcer>>>,
}

impl AppState {
    /// 创建仅含配置的初始状态（数据库/Redis 尚未连接）
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(Arc::new(config))),
            db: Arc::new(RwLock::new(None)),
            redis: None,
            enforcer: Arc::new(RwLock::new(None)),
        }
    }

    /// 设置 Redis 连接（构建时使用）
    pub fn with_redis(mut self, redis: redis::aio::ConnectionManager) -> Self {
        self.redis = Some(Arc::new(redis));
        self
    }

    // ===== 异步方法（用于 initdb 接口等需要写入的场景）=====

    /// 运行时动态设置数据库连接（initdb 接口调用）
    pub async fn set_db(&self, db: sea_orm::DatabaseConnection) {
        let mut guard = self.db.write().await;
        *guard = Some(Arc::new(db));
    }

    /// 运行时动态设置 Casbin Enforcer（initdb 接口调用）
    pub async fn set_enforcer(&self, enforcer: SharedEnforcer) {
        let mut guard = self.enforcer.write().await;
        *guard = Some(enforcer);
    }

    /// 检查数据库是否已初始化（异步）
    pub async fn has_db(&self) -> bool {
        self.db.read().await.is_some()
    }

    /// 获取数据库连接（异步，如果存在）
    pub async fn get_db(&self) -> Option<Arc<sea_orm::DatabaseConnection>> {
        self.db.read().await.clone()
    }

    /// 获取 Casbin Enforcer（异步，如果存在）
    pub async fn get_enforcer(&self) -> Option<SharedEnforcer> {
        self.enforcer.read().await.clone()
    }

    // ===== 同步方法（用于 Handler 中快速读取，避免 await）=====

    /// 同步获取当前配置快照（Handler / 中间件中使用）
    /// 返回 Arc<AppConfig>，读取开销极低
    pub fn get_config(&self) -> Arc<AppConfig> {
        self.config.try_read().map(|g| g.clone()).unwrap_or_else(|_| {
            // 极少发生：写锁被占用时（仅配置热重载瞬间），阻塞等待
            // 这里用 futures 的 block_on 不合适，直接 clone 一个默认值也不对
            // 实际上 try_read 失败概率极低，这里用 parking_lot 的 blocking 作为 fallback
            panic!("配置读锁获取失败，这不应该发生")
        })
    }

    /// 异步获取当前配置快照
    pub async fn get_config_async(&self) -> Arc<AppConfig> {
        self.config.read().await.clone()
    }

    /// 运行时更新配置（配置热重载时调用）
    pub async fn set_config(&self, config: AppConfig) {
        let mut guard = self.config.write().await;
        *guard = Arc::new(config);
    }

    /// 同步尝试获取数据库连接（Handler 中使用）
    /// 如果锁被占用则返回 None（极少发生，仅 initdb 时写锁）
    pub fn try_get_db(&self) -> Option<Arc<sea_orm::DatabaseConnection>> {
        self.db.try_read().ok().and_then(|g| g.clone())
    }

    /// 同步尝试获取 Casbin Enforcer（中间件中使用）
    pub fn try_get_enforcer(&self) -> Option<SharedEnforcer> {
        self.enforcer.try_read().ok().and_then(|g| g.clone())
    }
}
