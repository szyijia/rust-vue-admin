// 字典详情服务 - 对应 Gin-Vue-Admin server/service/system/sys_dictionary_detail.go
use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::model::system::sys_dictionary_detail;
use crate::model::system::sys_dictionary_detail::DictionaryDetailTreeNode;

/// 创建字典详情（对应 Go CreateSysDictionaryDetail）
/// 包含计算 level 和 path 的逻辑
pub async fn create_dictionary_detail(
    db: &DatabaseConnection,
    label: String,
    value: String,
    extend: String,
    status: bool,
    sort: i64,
    sys_dictionary_id: u64,
    parent_id: Option<u64>,
) -> Result<sys_dictionary_detail::Model> {
    let now = chrono::Local::now().naive_local();

    // 计算层级和路径（对应 Go 的逻辑）
    let (level, path) = if let Some(pid) = parent_id {
        let parent = sys_dictionary_detail::Entity::find_by_id(pid)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("父级字典详情不存在"))?;
        let parent_level = parent.level.unwrap_or(0);
        let parent_path = parent.path.clone().unwrap_or_default();
        let new_level = parent_level + 1;
        let new_path = if parent_path.is_empty() {
            format!("{}", parent.id)
        } else {
            format!("{},{}", parent_path, parent.id)
        };
        (Some(new_level), Some(new_path))
    } else {
        (Some(0), Some(String::new()))
    };

    let model = sys_dictionary_detail::ActiveModel {
        label: Set(label),
        value: Set(value),
        extend: Set(extend),
        status: Set(status),
        sort: Set(sort),
        sys_dictionary_id: Set(sys_dictionary_id),
        parent_id: Set(parent_id),
        level: Set(level),
        path: Set(path),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    let result = model.insert(db).await?;
    Ok(result)
}

/// 删除字典详情（对应 Go DeleteSysDictionaryDetail）
/// 检查是否有子项，有则拒绝删除
pub async fn delete_dictionary_detail(db: &DatabaseConnection, id: u64) -> Result<()> {
    // 检查是否有子项
    let count = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::ParentId.eq(id))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .count(db)
        .await?;
    if count > 0 {
        return Err(anyhow::anyhow!("该字典详情下还有子项，无法删除"));
    }
    sys_dictionary_detail::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 更新字典详情（对应 Go UpdateSysDictionaryDetail）
/// 包含重新计算 level/path 和更新子项的逻辑
pub async fn update_dictionary_detail(
    db: &DatabaseConnection,
    id: u64,
    label: String,
    value: String,
    extend: String,
    status: bool,
    sort: i64,
    parent_id: Option<u64>,
) -> Result<()> {
    let detail = sys_dictionary_detail::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典详情不存在"))?;

    // 计算层级和路径
    let (level, path) = if let Some(pid) = parent_id {
        // 检查循环引用
        if check_circular_reference(db, id, pid).await? {
            return Err(anyhow::anyhow!("不能将字典详情设置为自己或其子项的父级"));
        }
        let parent = sys_dictionary_detail::Entity::find_by_id(pid)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("父级字典详情不存在"))?;
        let parent_level = parent.level.unwrap_or(0);
        let parent_path = parent.path.clone().unwrap_or_default();
        let new_level = parent_level + 1;
        let new_path = if parent_path.is_empty() {
            format!("{}", parent.id)
        } else {
            format!("{},{}", parent_path, parent.id)
        };
        (Some(new_level), Some(new_path))
    } else {
        (Some(0), Some(String::new()))
    };

    let now = chrono::Local::now().naive_local();
    let mut active: sys_dictionary_detail::ActiveModel = detail.into();
    active.label = Set(label);
    active.value = Set(value);
    active.extend = Set(extend);
    active.status = Set(status);
    active.sort = Set(sort);
    active.parent_id = Set(parent_id);
    active.level = Set(level);
    active.path = Set(path);
    active.updated_at = Set(Some(now));
    active.update(db).await?;

    // 更新所有子项的层级和路径
    update_children_level_and_path(db, id).await?;

    Ok(())
}

/// 检查循环引用（对应 Go checkCircularReference）
fn check_circular_reference(db: &DatabaseConnection, id: u64, parent_id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool>> + Send + '_>> {
    Box::pin(async move {
        if id == parent_id {
            return Ok(true);
        }
        let parent = sys_dictionary_detail::Entity::find_by_id(parent_id)
            .one(db)
            .await?;
        match parent {
            Some(p) => {
                if let Some(pid) = p.parent_id {
                    check_circular_reference(db, id, pid).await
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    })
}

/// 更新子项的层级和路径（对应 Go updateChildrenLevelAndPath）
async fn update_children_level_and_path(db: &DatabaseConnection, parent_id: u64) -> Result<()> {
    let parent = sys_dictionary_detail::Entity::find_by_id(parent_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("父级不存在"))?;

    let children = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::ParentId.eq(parent_id))
        .all(db)
        .await?;

    let parent_level = parent.level.unwrap_or(0);
    let parent_path = parent.path.clone().unwrap_or_default();

    for child in children {
        let new_level = parent_level + 1;
        let new_path = if parent_path.is_empty() {
            format!("{}", parent.id)
        } else {
            format!("{},{}", parent_path, parent.id)
        };

        let mut active: sys_dictionary_detail::ActiveModel = child.into();
        active.level = Set(Some(new_level));
        active.path = Set(Some(new_path));
        let saved = active.update(db).await?;

        // 递归更新子项的子项
        Box::pin(update_children_level_and_path(db, saved.id)).await?;
    }

    Ok(())
}

/// 根据 ID 获取字典详情
pub async fn find_dictionary_detail(
    db: &DatabaseConnection,
    id: u64,
) -> Result<sys_dictionary_detail::Model> {
    sys_dictionary_detail::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典详情不存在"))
}

/// 分页查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailListResult {
    pub list: Vec<sys_dictionary_detail::Model>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 获取字典详情列表（分页）
pub async fn get_dictionary_detail_list(
    db: &DatabaseConnection,
    sys_dictionary_id: u64,
    label: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<DetailListResult> {
    let mut query = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(sys_dictionary_id))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null());

    if let Some(ref l) = label {
        if !l.is_empty() {
            query = query.filter(sys_dictionary_detail::Column::Label.contains(l));
        }
    }

    let total = query.clone().count(db).await? as i64;

    let list = query
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .offset(((page - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    Ok(DetailListResult {
        list,
        total,
        page,
        page_size,
    })
}

/// 获取字典树形结构列表（对应 Go GetDictionaryTreeList）
/// GET /sysDictionaryDetail/getDictionaryTreeList?sysDictionaryID=xxx
pub async fn get_dictionary_tree_list(
    db: &DatabaseConnection,
    dictionary_id: u64,
) -> Result<Vec<DictionaryDetailTreeNode>> {
    // 只获取顶级项目（parent_id为空）
    let top_items = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(dictionary_id))
        .filter(sys_dictionary_detail::Column::ParentId.is_null())
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;

    let mut result = Vec::new();
    for item in &top_items {
        let mut node = DictionaryDetailTreeNode::from_model(item);
        load_children(db, &mut node).await?;
        result.push(node);
    }

    Ok(result)
}

/// 根据字典类型获取树形结构（对应 Go GetDictionaryTreeListByType）
/// GET /sysDictionaryDetail/getDictionaryTreeListByType?type=xxx
pub async fn get_dictionary_tree_list_by_type(
    db: &DatabaseConnection,
    dict_type: &str,
) -> Result<Vec<DictionaryDetailTreeNode>> {
    use crate::model::system::sys_dictionary;
    // 先找到字典 ID
    let dict = sys_dictionary::Entity::find()
        .filter(sys_dictionary::Column::Type.eq(dict_type))
        .filter(sys_dictionary::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典类型不存在: {}", dict_type))?;

    // 获取该字典下的顶级项
    let top_items = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(dict.id))
        .filter(sys_dictionary_detail::Column::ParentId.is_null())
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;

    let mut result = Vec::new();
    for item in &top_items {
        let mut node = DictionaryDetailTreeNode::from_model(item);
        load_children(db, &mut node).await?;
        result.push(node);
    }

    Ok(result)
}

/// 根据父级ID获取字典详情（对应 Go GetDictionaryDetailsByParent）
/// GET /sysDictionaryDetail/getDictionaryDetailsByParent?sysDictionaryID=xxx&parentID=xxx&includeChildren=true
pub async fn get_dictionary_details_by_parent(
    db: &DatabaseConnection,
    sys_dictionary_id: u64,
    parent_id: Option<u64>,
    include_children: bool,
) -> Result<Vec<DictionaryDetailTreeNode>> {
    let mut query = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::SysDictionaryId.eq(sys_dictionary_id))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null());

    if let Some(pid) = parent_id {
        query = query.filter(sys_dictionary_detail::Column::ParentId.eq(pid));
    } else {
        query = query.filter(sys_dictionary_detail::Column::ParentId.is_null());
    }

    let items = query
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;

    let mut result = Vec::new();
    for item in &items {
        let mut node = DictionaryDetailTreeNode::from_model(item);
        if include_children {
            load_children(db, &mut node).await?;
        }
        result.push(node);
    }

    Ok(result)
}

/// 获取字典详情的完整路径（对应 Go GetDictionaryPath）
/// GET /sysDictionaryDetail/getDictionaryPath?id=xxx
/// 返回从根到当前节点的路径数组
pub async fn get_dictionary_path(
    db: &DatabaseConnection,
    id: u64,
) -> Result<Vec<sys_dictionary_detail::Model>> {
    let detail = sys_dictionary_detail::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("字典详情不存在"))?;

    let mut path = Vec::new();

    if let Some(pid) = detail.parent_id {
        // 递归获取父级路径
        let parent_path = Box::pin(get_dictionary_path(db, pid)).await?;
        path.extend(parent_path);
    }

    path.push(detail);
    Ok(path)
}

/// 递归加载子项（对应 Go loadChildren）
async fn load_children(db: &DatabaseConnection, node: &mut DictionaryDetailTreeNode) -> Result<()> {
    let children = sys_dictionary_detail::Entity::find()
        .filter(sys_dictionary_detail::Column::ParentId.eq(node.id))
        .filter(sys_dictionary_detail::Column::DeletedAt.is_null())
        .order_by_asc(sys_dictionary_detail::Column::Sort)
        .all(db)
        .await?;

    for child in &children {
        let mut child_node = DictionaryDetailTreeNode::from_model(child);
        Box::pin(load_children(db, &mut child_node)).await?;
        node.children.push(child_node);
    }

    Ok(())
}
