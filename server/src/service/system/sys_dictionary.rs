use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::model::system::{sys_dictionary, sys_dictionary_detail};

/// 字典信息（含字典详情）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: bool,
    pub desc: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sys_dictionary_details: Vec<sys_dictionary_detail::Model>,
}

impl From<sys_dictionary::Model> for DictionaryInfo {
    fn from(m: sys_dictionary::Model) -> Self {
        Self {
            id: m.id,
            created_at: m.created_at,
            updated_at: m.updated_at,
            name: m.name,
            r#type: m.r#type,
            status: m.status,
            desc: m.desc,
            sys_dictionary_details: vec![],
        }
    }
}

/// 创建字典
pub async fn create_dictionary(
    db: &DatabaseConnection,
    name: String,
    r#type: String,
    status: bool,
    desc: String,
) -> Result<()> {
    // 检查 type 是否已存在
    let existing = sys_dictionary::Entity::find()
        .filter(sys_dictionary::Column::Type.eq(&r#type))
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("存在相同的type，不允许创建"));
    }

    let now = chrono::Local::now().naive_local();
    let model = sys_dictionary::ActiveModel {
        name: Set(name),
        r#type: Set(r#type),
        status: Set(status),
        desc: Set(desc),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

/// 删除字典（同时删除字典详情）
pub async fn delete_dictionary(db: &DatabaseConnection, id: u64) -> Result<()> {
    let dict = sys_dictionary::Entity::find_by_id(id)
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典不存在"))?;

    let txn = db.begin().await?;

    // 软删除字典
    let now = chrono::Local::now().naive_local();
    let mut active: sys_dictionary::ActiveModel = dict.into();
    active.deleted_at = Set(Some(now));
    active.update(&txn).await?;

    // 删除对应的字典详情
    sys_dictionary_detail::Entity::delete_many()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(id))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(())
}

/// 更新字典
pub async fn update_dictionary(
    db: &DatabaseConnection,
    id: u64,
    name: String,
    r#type: String,
    status: bool,
    desc: String,
) -> Result<()> {
    let dict = sys_dictionary::Entity::find_by_id(id)
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典不存在"))?;

    // 如果 type 发生变化，检查新 type 是否已存在
    if dict.r#type != r#type {
        let existing = sys_dictionary::Entity::find()
            .filter(sys_dictionary::Column::Type.eq(&r#type))
            .filter(sys_dictionary::Column::DeletedAt.is_null())
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(anyhow::anyhow!("存在相同的type，不允许创建"));
        }
    }

    let now = chrono::Local::now().naive_local();
    let mut active: sys_dictionary::ActiveModel = dict.into();
    active.name = Set(name);
    active.r#type = Set(r#type);
    active.status = Set(status);
    active.desc = Set(desc);
    active.updated_at = Set(Some(now));
    active.update(db).await?;
    Ok(())
}

/// 根据 ID 或 type 查询字典（含详情）
pub async fn find_dictionary(
    db: &DatabaseConnection,
    r#type: Option<String>,
    id: Option<u64>,
    status: Option<bool>,
) -> Result<DictionaryInfo> {
    let flag = status.unwrap_or(true);

    let mut query = sys_dictionary::Entity::find()
        .filter(sys_dictionary::Column::Status.eq(flag))
        .filter(sys_dictionary::Column::DeletedAt.is_null());

    if let Some(t) = &r#type {
        if !t.is_empty() {
            query = query.filter(sys_dictionary::Column::Type.eq(t));
        }
    }
    if let Some(id) = id {
        if id > 0 {
            query = query.filter(sys_dictionary::Column::Id.eq(id));
        }
    }

    let dict = query.one(db).await?
        .ok_or_else(|| anyhow::anyhow!("字典未创建或未开启"))?;

    let dict_id = dict.id;
    let mut info: DictionaryInfo = dict.into();

    // 加载字典详情
    let details = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(dict_id))
        .filter(sys_dictionary_detail::Column::Status.eq(true))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;
    info.sys_dictionary_details = details;

    Ok(info)
}

/// 获取字典列表
pub async fn get_dictionary_list(
    db: &DatabaseConnection,
    name: Option<String>,
) -> Result<Vec<DictionaryInfo>> {
    let mut query = sys_dictionary::Entity::find()
        .filter(sys_dictionary::Column::DeletedAt.is_null());

    if let Some(ref n) = name {
        if !n.is_empty() {
            query = query.filter(
                sea_orm::Condition::any()
                    .add(sys_dictionary::Column::Name.contains(n))
                    .add(sys_dictionary::Column::Type.contains(n)),
            );
        }
    }

    let list = query.order_by_asc(sys_dictionary::Column::Id).all(db).await?;

    let result: Vec<DictionaryInfo> = list.into_iter().map(DictionaryInfo::from).collect();
    Ok(result)
}

/// 导出字典JSON（含字典详情）
pub async fn export_dictionary(db: &DatabaseConnection, id: u64) -> Result<serde_json::Value> {
    let dict = sys_dictionary::Entity::find_by_id(id)
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典不存在"))?;

    let details = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(id))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;

    let clean_details: Vec<serde_json::Value> = details.iter().map(|d| {
        serde_json::json!({
            "label": d.label,
            "value": d.value,
            "extend": d.extend,
            "status": d.status,
            "sort": d.sort,
        })
    }).collect();

    Ok(serde_json::json!({
        "name": dict.name,
        "type": dict.r#type,
        "status": dict.status,
        "desc": dict.desc,
        "sysDictionaryDetails": clean_details,
    }))
}

/// 导入字典JSON
pub async fn import_dictionary(db: &DatabaseConnection, json_str: &str) -> Result<()> {
    let import_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow::anyhow!("JSON 格式错误: {}", e))?;

    let name = import_data["name"].as_str().unwrap_or("").to_string();
    let r#type = import_data["type"].as_str().unwrap_or("").to_string();
    if name.is_empty() {
        return Err(anyhow::anyhow!("字典名称不能为空"));
    }
    if r#type.is_empty() {
        return Err(anyhow::anyhow!("字典类型不能为空"));
    }

    // 检查 type 是否已存在
    let existing = sys_dictionary::Entity::find()
        .filter(sys_dictionary::Column::Type.eq(&r#type))
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    if existing.is_some() {
        return Err(anyhow::anyhow!("存在相同的type，不允许导入"));
    }

    let txn = db.begin().await?;
    let now = chrono::Local::now().naive_local();

    let status = import_data["status"].as_bool().unwrap_or(true);
    let desc = import_data["desc"].as_str().unwrap_or("").to_string();

    let dict = sys_dictionary::ActiveModel {
        name: Set(name),
        r#type: Set(r#type),
        status: Set(status),
        desc: Set(desc),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    let dict = dict.insert(&txn).await?;

    // 处理字典详情
    if let Some(details) = import_data["sysDictionaryDetails"].as_array() {
        for detail in details {
            let label = detail["label"].as_str().unwrap_or("").to_string();
            let value = detail["value"].as_str().unwrap_or("").to_string();
            if label.is_empty() || value.is_empty() {
                continue;
            }
            let d = sys_dictionary_detail::ActiveModel {
                label: Set(label),
                value: Set(value),
                extend: Set(detail["extend"].as_str().unwrap_or("").to_string()),
                status: Set(detail["status"].as_bool().unwrap_or(true)),
                sort: Set(detail["sort"].as_i64().unwrap_or(0)),
                sys_dictionary_id: Set(dict.id),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                ..Default::default()
            };
            d.insert(&txn).await?;
        }
    }

    txn.commit().await?;
    Ok(())
}
