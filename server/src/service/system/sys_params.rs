// 参数管理服务
use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::sys_params;

/// 创建参数
pub async fn create_params(
    db: &DatabaseConnection,
    name: String,
    key: String,
    value: String,
    desc: String,
) -> Result<()> {
    // 检查 key 是否已存在
    let existing = sys_params::Entity::find()
        .filter(sys_params::Column::Key.eq(&key))
        .filter(sys_params::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("参数key已存在"));
    }

    let now = chrono::Local::now().naive_local();
    let model = sys_params::ActiveModel {
        name: Set(name),
        key: Set(key),
        value: Set(value),
        desc: Set(desc),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 删除参数
pub async fn delete_params(db: &DatabaseConnection, id: u64) -> Result<()> {
    sys_params::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 批量删除参数
pub async fn delete_params_by_ids(db: &DatabaseConnection, ids: Vec<u64>) -> Result<()> {
    sys_params::Entity::delete_many()
        .filter(sys_params::Column::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 更新参数
pub async fn update_params(
    db: &DatabaseConnection,
    id: u64,
    name: String,
    key: String,
    value: String,
    desc: String,
) -> Result<()> {
    let param = sys_params::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("参数不存在"))?;

    let now = chrono::Local::now().naive_local();
    let mut active: sys_params::ActiveModel = param.into();
    active.name = Set(name);
    active.key = Set(key);
    active.value = Set(value);
    active.desc = Set(desc);
    active.updated_at = Set(Some(now));
    active.update(db).await?;
    Ok(())
}

/// 根据 ID 获取参数
pub async fn find_params(
    db: &DatabaseConnection,
    id: u64,
) -> Result<sys_params::Model> {
    sys_params::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("参数不存在"))
}

/// 根据 key 获取参数值
pub async fn get_params_by_key(
    db: &DatabaseConnection,
    key: &str,
) -> Result<sys_params::Model> {
    sys_params::Entity::find()
        .filter(sys_params::Column::Key.eq(key))
        .filter(sys_params::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("参数不存在"))
}

/// 分页查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamsListResult {
    pub list: Vec<sys_params::Model>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 获取参数列表（分页）
pub async fn get_params_list(
    db: &DatabaseConnection,
    name: Option<String>,
    key: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<ParamsListResult> {
    let mut query = sys_params::Entity::find()
        .filter(sys_params::Column::DeletedAt.is_null());

    if let Some(ref n) = name {
        if !n.is_empty() {
            query = query.filter(sys_params::Column::Name.contains(n));
        }
    }
    if let Some(ref k) = key {
        if !k.is_empty() {
            query = query.filter(sys_params::Column::Key.contains(k));
        }
    }

    let total = query.clone().count(db).await? as i64;

    let list = query
        .order_by_desc(sys_params::Column::Id)
        .offset(((page - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    Ok(ParamsListResult {
        list,
        total,
        page,
        page_size,
    })
}
