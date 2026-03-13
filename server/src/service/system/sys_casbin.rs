use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{initialize::casbin::SharedEnforcer, model::system::casbin_rule};

/// Casbin 策略信息（路径 + 方法）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CasbinInfo {
    pub path: String,
    pub method: String,
}

/// 更新角色的 API 权限（全量替换）
/// 对应 Gin-Vue-Admin 的 casbinService.UpdateCasbin()
pub async fn update_casbin(
    db: &DatabaseConnection,
    enforcer: &SharedEnforcer,
    _admin_role_id: u64,
    role_id: u64,
    policies: Vec<CasbinInfo>,
) -> Result<()> {
    // 超级管理员不允许修改权限
    if role_id == 888 {
        return Err(anyhow::anyhow!("超级管理员权限不可修改"));
    }

    let role_str = role_id.to_string();

    // 删除该角色的所有旧策略
    casbin_rule::Entity::delete_many()
        .filter(casbin_rule::Column::Ptype.eq("p"))
        .filter(casbin_rule::Column::V0.eq(&role_str))
        .exec(db)
        .await?;

    // 插入新策略
    for policy in &policies {
        let rule = casbin_rule::ActiveModel {
            ptype: Set("p".to_string()),
            v0: Set(role_str.clone()),
            v1: Set(policy.path.clone()),
            v2: Set(policy.method.clone()),
            v3: Set("".to_string()),
            v4: Set("".to_string()),
            v5: Set("".to_string()),
            ..Default::default()
        };
        rule.insert(db).await?;
    }

    // 刷新 Casbin 内存策略
    crate::initialize::casbin::fresh_casbin(enforcer, db).await?;

    Ok(())
}

/// 获取角色的所有 API 权限
/// 对应 Gin-Vue-Admin 的 casbinService.GetPolicyPathByAuthorityId()
pub async fn get_policy_by_role(
    db: &DatabaseConnection,
    role_id: u64,
) -> Result<Vec<CasbinInfo>> {
    let role_str = role_id.to_string();

    let rules = casbin_rule::Entity::find()
        .filter(casbin_rule::Column::Ptype.eq("p"))
        .filter(casbin_rule::Column::V0.eq(&role_str))
        .all(db)
        .await?;

    let policies = rules
        .into_iter()
        .map(|r| CasbinInfo {
            path: r.v1,
            method: r.v2,
        })
        .collect();

    Ok(policies)
}

/// 刷新 Casbin 缓存
pub async fn fresh_casbin_cache(
    enforcer: &SharedEnforcer,
    db: &DatabaseConnection,
) -> Result<()> {
    crate::initialize::casbin::fresh_casbin(enforcer, db).await
}

/// 获取拥有指定 API 权限的所有角色 ID，对应 Gin-Vue-Admin 的 casbinService.GetAuthoritiesByApi()
pub async fn get_authorities_by_api(
    db: &DatabaseConnection,
    path: &str,
    method: &str,
) -> Result<Vec<u64>> {
    let rules = casbin_rule::Entity::find()
        .filter(casbin_rule::Column::Ptype.eq("p"))
        .filter(casbin_rule::Column::V1.eq(path))
        .filter(casbin_rule::Column::V2.eq(method))
        .all(db)
        .await?;

    let authority_ids: Vec<u64> = rules
        .into_iter()
        .filter_map(|r| r.v0.parse::<u64>().ok())
        .collect();

    Ok(authority_ids)
}

/// 全量覆盖某 API 关联的角色列表，对应 Gin-Vue-Admin 的 casbinService.SetApiAuthorities()
pub async fn set_api_authorities(
    db: &DatabaseConnection,
    path: &str,
    method: &str,
    authority_ids: Vec<u64>,
) -> Result<()> {
    use sea_orm::TransactionTrait;
    let txn = db.begin().await?;

    // 1. 删除该 API 所有已有的角色关联
    casbin_rule::Entity::delete_many()
        .filter(casbin_rule::Column::Ptype.eq("p"))
        .filter(casbin_rule::Column::V1.eq(path))
        .filter(casbin_rule::Column::V2.eq(method))
        .exec(&txn)
        .await?;

    // 2. 批量插入新的关联记录
    for authority_id in authority_ids {
        let rule = casbin_rule::ActiveModel {
            ptype: Set("p".to_string()),
            v0: Set(authority_id.to_string()),
            v1: Set(path.to_string()),
            v2: Set(method.to_string()),
            v3: Set("".to_string()),
            v4: Set("".to_string()),
            v5: Set("".to_string()),
            ..Default::default()
        };
        rule.insert(&txn).await?;
    }

    txn.commit().await?;
    Ok(())
}
