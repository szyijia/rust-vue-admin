use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 字典详情表，对应 Gin-Vue-Admin 的 SysDictionaryDetail
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_dictionary_details")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
    /// 展示值
    pub label: String,
    /// 字典值
    pub value: String,
    /// 扩展值
    pub extend: String,
    /// 启用状态
    pub status: bool,
    /// 排序标记
    pub sort: i64,
    /// 关联字典ID
    pub sys_dictionary_id: u64,
    /// 父级字典详情ID
    #[serde(rename = "parentID")]
    pub parent_id: Option<u64>,
    /// 层级深度，从0开始
    pub level: Option<i64>,
    /// 层级路径，如 "1,2,3"
    #[sea_orm(column_name = "path")]
    #[serde(rename = "path")]
    pub path: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 树形字典详情节点（用于 API 返回，含 children 和 disabled 字段）
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryDetailTreeNode {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    pub label: String,
    pub value: String,
    pub extend: String,
    pub status: bool,
    pub sort: i64,
    pub sys_dictionary_id: u64,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u64>,
    pub level: Option<i64>,
    pub path: Option<String>,
    /// 禁用状态，根据 status 字段动态计算（status 为 false 时 disabled 为 true）
    pub disabled: bool,
    /// 子字典详情
    pub children: Vec<DictionaryDetailTreeNode>,
}

impl DictionaryDetailTreeNode {
    /// 从 Model 创建树节点（不含 children，需后续填充）
    pub fn from_model(m: &Model) -> Self {
        Self {
            id: m.id,
            created_at: m.created_at,
            updated_at: m.updated_at,
            label: m.label.clone(),
            value: m.value.clone(),
            extend: m.extend.clone(),
            status: m.status,
            sort: m.sort,
            sys_dictionary_id: m.sys_dictionary_id,
            parent_id: m.parent_id,
            level: m.level,
            path: m.path.clone(),
            disabled: !m.status,
            children: Vec::new(),
        }
    }
}
