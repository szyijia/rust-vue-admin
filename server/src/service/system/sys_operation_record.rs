// 操作记录服务
use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::sys_operation_record;

/// 创建操作记录
pub async fn create_operation_record(
    db: &DatabaseConnection,
    ip: String,
    method: String,
    path: String,
    status: i64,
    latency: i64,
    agent: String,
    error_message: String,
    body: String,
    resp: String,
    user_id: u64,
) -> Result<()> {
    let now = chrono::Local::now().naive_local();
    let model = sys_operation_record::ActiveModel {
        ip: Set(ip),
        method: Set(method),
        path: Set(path),
        status: Set(status),
        latency: Set(latency),
        agent: Set(agent),
        error_message: Set(error_message),
        body: Set(body),
        resp: Set(resp),
        user_id: Set(user_id),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 分页查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecordListResult {
    pub list: Vec<sys_operation_record::Model>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 获取操作记录列表（分页）
pub async fn get_operation_record_list(
    db: &DatabaseConnection,
    method: Option<String>,
    path: Option<String>,
    status: Option<i32>,
    page: i64,
    page_size: i64,
) -> Result<OperationRecordListResult> {
    let mut query = sys_operation_record::Entity::find()
        .filter(sys_operation_record::Column::DeletedAt.is_null());

    if let Some(ref m) = method {
        if !m.is_empty() {
            query = query.filter(sys_operation_record::Column::Method.eq(m));
        }
    }
    if let Some(ref p) = path {
        if !p.is_empty() {
            query = query.filter(sys_operation_record::Column::Path.contains(p));
        }
    }
    if let Some(s) = status {
        query = query.filter(sys_operation_record::Column::Status.eq(s));
    }

    let total = query.clone().count(db).await? as i64;

    let list = query
        .order_by_desc(sys_operation_record::Column::Id)
        .offset(((page - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    Ok(OperationRecordListResult {
        list,
        total,
        page,
        page_size,
    })
}

/// 删除操作记录
pub async fn delete_operation_record(db: &DatabaseConnection, id: u64) -> Result<()> {
    sys_operation_record::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 批量删除操作记录
pub async fn delete_operation_records_by_ids(db: &DatabaseConnection, ids: Vec<u64>) -> Result<()> {
    sys_operation_record::Entity::delete_many()
        .filter(sys_operation_record::Column::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 根据 ID 获取操作记录
pub async fn find_operation_record(
    db: &DatabaseConnection,
    id: u64,
) -> Result<sys_operation_record::Model> {
    sys_operation_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("操作记录不存在"))
}
