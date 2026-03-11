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
    _admin_role_id: i64,
    role_id: i64,
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
    role_id: i64,
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
