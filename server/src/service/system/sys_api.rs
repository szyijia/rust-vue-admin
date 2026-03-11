use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::sys_api;

/// API 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub id: i32,
    pub path: String,
    pub description: String,
    pub api_group: String,
    pub method: String,
}

impl From<sys_api::Model> for ApiInfo {
    fn from(m: sys_api::Model) -> Self {
        Self {
            id: m.id,
            path: m.path,
            description: m.description,
            api_group: m.api_group,
            method: m.method,
        }
    }
}

/// 创建 API
pub async fn create_api(
    db: &DatabaseConnection,
    path: String,
    description: String,
    api_group: String,
    method: String,
) -> Result<()> {
    // 检查是否已存在相同路径+方法
    let existing = sys_api::Entity::find()
        .filter(sys_api::Column::Path.eq(&path))
        .filter(sys_api::Column::Method.eq(&method))
        .one(db)
        .await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("API {} [{}] 已存在", path, method));
    }

    let model = sys_api::ActiveModel {
        path: Set(path),
        description: Set(description),
        api_group: Set(api_group),
        method: Set(method),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 删除 API
pub async fn delete_api(db: &DatabaseConnection, id: i32) -> Result<()> {
    sys_api::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 批量删除 API
pub async fn delete_apis_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<()> {
    sys_api::Entity::delete_many()
        .filter(sys_api::Column::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 更新 API
pub async fn update_api(
    db: &DatabaseConnection,
    id: i32,
    path: String,
    description: String,
    api_group: String,
    method: String,
) -> Result<()> {
    let model = sys_api::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("API 不存在"))?;

    let mut active: sys_api::ActiveModel = model.into();
    active.path = Set(path);
    active.description = Set(description);
    active.api_group = Set(api_group);
    active.method = Set(method);
    active.update(db).await?;
    Ok(())
}

/// 分页获取 API 列表
pub async fn get_api_list(
    db: &DatabaseConnection,
    page: i64,
    page_size: i64,
    path_filter: Option<String>,
    group_filter: Option<String>,
    method_filter: Option<String>,
) -> Result<(Vec<ApiInfo>, u64)> {
    use sea_orm::PaginatorTrait;

    let mut query = sys_api::Entity::find()
        .order_by_asc(sys_api::Column::ApiGroup)
        .order_by_asc(sys_api::Column::Path);

    if let Some(path) = path_filter {
        if !path.is_empty() {
            query = query.filter(sys_api::Column::Path.contains(&path));
        }
    }
    if let Some(group) = group_filter {
        if !group.is_empty() {
            query = query.filter(sys_api::Column::ApiGroup.eq(&group));
        }
    }
    if let Some(method) = method_filter {
        if !method.is_empty() {
            query = query.filter(sys_api::Column::Method.eq(&method));
        }
    }

    let paginator = query.paginate(db, page_size as u64);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page((page - 1) as u64).await?;

    Ok((list.into_iter().map(ApiInfo::from).collect(), total))
}

/// 获取所有 API（不分页）
pub async fn get_all_apis(db: &DatabaseConnection) -> Result<Vec<ApiInfo>> {
    let list = sys_api::Entity::find()
        .order_by_asc(sys_api::Column::ApiGroup)
        .order_by_asc(sys_api::Column::Path)
        .all(db)
        .await?;
    Ok(list.into_iter().map(ApiInfo::from).collect())
}

/// 根据 ID 获取单个 API
pub async fn get_api_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<ApiInfo>> {
    let model = sys_api::Entity::find_by_id(id).one(db).await?;
    Ok(model.map(ApiInfo::from))
}
