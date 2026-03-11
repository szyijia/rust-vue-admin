use anyhow::Result;
use casbin::{CoreApi, DefaultModel, Enforcer, MgmtApi};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Casbin Enforcer 的共享类型
pub type SharedEnforcer = Arc<RwLock<Enforcer>>;

/// RBAC 模型定义（内嵌，无需外部文件）
const RBAC_MODEL: &str = r#"
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
"#;

/// 初始化 Casbin Enforcer（使用内存适配器，从数据库手动加载策略）
pub async fn init_casbin(db: &DatabaseConnection) -> Result<SharedEnforcer> {
    use casbin::MemoryAdapter;

    let model = DefaultModel::from_str(RBAC_MODEL).await?;
    let adapter = MemoryAdapter::default();
    let mut enforcer = Enforcer::new(model, adapter).await?;

    // 从数据库加载策略
    load_policies_from_db(&mut enforcer, db).await?;

    info!("✅ Casbin 初始化完成，已从数据库加载权限策略");
    Ok(Arc::new(RwLock::new(enforcer)))
}

/// 从数据库加载所有 Casbin 策略到 Enforcer
async fn load_policies_from_db(enforcer: &mut Enforcer, db: &DatabaseConnection) -> Result<()> {
    use sea_orm::EntityTrait;
    use crate::model::system::casbin_rule;

    let rules = casbin_rule::Entity::find().all(db).await?;

    for rule in rules {
        let tokens: Vec<String> = vec![rule.v0, rule.v1, rule.v2, rule.v3, rule.v4, rule.v5]
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect();
        if tokens.is_empty() { continue; }
        let _ = enforcer.add_policy(tokens).await;
    }
    Ok(())
}

/// 检查某角色是否有访问某路径+方法的权限
pub async fn check_permission(
    enforcer: &SharedEnforcer,
    role_id: u64,
    path: &str,
    method: &str,
) -> bool {
    let e = enforcer.read().await;
    match e.enforce(vec![role_id.to_string(), path.to_string(), method.to_string()]) {
        Ok(result) => result,
        Err(err) => {
            warn!("Casbin enforce 失败: {}", err);
            false
        }
    }
}

/// 刷新 Casbin 策略（从数据库重新加载）
pub async fn fresh_casbin(enforcer: &SharedEnforcer, db: &DatabaseConnection) -> Result<()> {
    use sea_orm::EntityTrait;
    use crate::model::system::casbin_rule;

    let rules = casbin_rule::Entity::find().all(db).await?;

    let mut e = enforcer.write().await;
    e.clear_policy().await?;

    for rule in rules {
        let tokens: Vec<String> = vec![rule.v0, rule.v1, rule.v2, rule.v3, rule.v4, rule.v5]
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect();
        if tokens.is_empty() { continue; }
        let _ = e.add_policy(tokens).await;
    }

    info!("✅ Casbin 策略已刷新");
    Ok(())
}
