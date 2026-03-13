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
    #[serde(rename = "ID")]
    pub id: u64,
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
pub async fn delete_api(db: &DatabaseConnection, id: u64) -> Result<()> {
    sys_api::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

/// 批量删除 API
pub async fn delete_apis_by_ids(db: &DatabaseConnection, ids: Vec<u64>) -> Result<()> {
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
    let model = sys_api::Entity::find_by_id(id as u64)
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
    let model = sys_api::Entity::find_by_id(id as u64).one(db).await?;
    Ok(model.map(ApiInfo::from))
}

// ===== API 同步相关 =====
use crate::model::system::sys_ignore_api;

/// 路由信息（用于 sync_api 对比）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteInfo {
    pub path: String,
    pub method: String,
}

/// 同步 API 结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncApiResult {
    pub new_apis: Vec<ApiInfo>,
    pub delete_apis: Vec<ApiInfo>,
    pub ignore_apis: Vec<ApiInfo>,
}

/// 获取已注册的路由列表
/// 由于 Rust/Axum 没有像 Go/Gin 那样的全局路由列表，
/// 需要通过硬编码方式维护一份路由列表。
/// 在每次添加新路由时，都需要同步更新此列表。
pub fn get_registered_routes() -> Vec<RouteInfo> {
    // 这里列出所有已注册的路由 (path, method)
    vec![
        // 公开路由
        RouteInfo { path: "/base/captcha".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/base/login".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/base/register".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/init/checkdb".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/init/initdb".to_string(), method: "POST".to_string() },
        // 用户
        RouteInfo { path: "/user/getUserInfo".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/user/setSelfInfo".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/user/changePassword".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/jwt/jsonInBlacklist".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/getUserList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/deleteUser".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/user/setUserAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/setUserInfo".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/user/resetPassword".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/setUserEnable".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/admin_register".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/setUserAuthorities".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/user/setSelfSetting".to_string(), method: "PUT".to_string() },
        // 角色
        RouteInfo { path: "/authority/createAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/deleteAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/updateAuthority".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/authority/getAuthorityList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/copyAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/setDataAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/setRoleUsers".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authority/getUsersByAuthority".to_string(), method: "GET".to_string() },
        // Casbin
        RouteInfo { path: "/casbin/UpdateCasbin".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/casbin/getPolicyPathByAuthorityId".to_string(), method: "POST".to_string() },
        // 菜单
        RouteInfo { path: "/menu/getMenu".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/getMenuList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/getBaseMenuTree".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/getBaseMenuById".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/addBaseMenu".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/deleteBaseMenu".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/updateBaseMenu".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/addMenuAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/getMenuAuthority".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/menu/getMenuRoles".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/menu/setMenuRoles".to_string(), method: "POST".to_string() },
        // API 管理
        RouteInfo { path: "/api/createApi".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/deleteApi".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/deleteApisByIds".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/api/updateApi".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/getApiList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/getAllApis".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/getApiById".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/getApiGroups".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/api/syncApi".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/api/ignoreApi".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/enterSyncApi".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/getApiRoles".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/api/setApiRoles".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/api/freshCasbin".to_string(), method: "GET".to_string() },
        // 字典
        RouteInfo { path: "/sysDictionary/createSysDictionary".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/sysDictionary/deleteSysDictionary".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysDictionary/updateSysDictionary".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/sysDictionary/findSysDictionary".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/sysDictionary/getSysDictionaryList".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/sysDictionary/exportSysDictionary".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/sysDictionary/importSysDictionary".to_string(), method: "POST".to_string() },
        // 字典详情
        RouteInfo { path: "/sysDictionaryDetail/createSysDictionaryDetail".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/sysDictionaryDetail/deleteSysDictionaryDetail".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysDictionaryDetail/updateSysDictionaryDetail".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/sysDictionaryDetail/findSysDictionaryDetail".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/sysDictionaryDetail/getSysDictionaryDetailList".to_string(), method: "GET".to_string() },
        // 操作记录
        RouteInfo { path: "/sysOperationRecord/getSysOperationRecordList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/sysOperationRecord/deleteSysOperationRecord".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysOperationRecord/deleteSysOperationRecordByIds".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysOperationRecord/findSysOperationRecord".to_string(), method: "GET".to_string() },
        // 参数管理
        RouteInfo { path: "/sysParams/createSysParams".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/sysParams/deleteSysParams".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysParams/deleteSysParamsByIds".to_string(), method: "DELETE".to_string() },
        RouteInfo { path: "/sysParams/updateSysParams".to_string(), method: "PUT".to_string() },
        RouteInfo { path: "/sysParams/findSysParams".to_string(), method: "GET".to_string() },
        RouteInfo { path: "/sysParams/getSysParamsList".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/sysParams/getSysParamsByKey".to_string(), method: "GET".to_string() },
        // 按钮权限
        RouteInfo { path: "/authorityBtn/getAuthorityBtn".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authorityBtn/setAuthorityBtn".to_string(), method: "POST".to_string() },
        RouteInfo { path: "/authorityBtn/canRemoveAuthorityBtn".to_string(), method: "POST".to_string() },
    ]
}

/// 同步 API：对比已注册路由和数据库中的 API，对应 Gin-Vue-Admin 的 apiService.SyncApi()
pub async fn sync_api(db: &DatabaseConnection) -> Result<SyncApiResult> {
    // 从数据库获取所有 API
    let db_apis = sys_api::Entity::find().all(db).await?;

    // 获取忽略列表
    let ignores = sys_ignore_api::Entity::find().all(db).await?;

    let ignore_apis: Vec<ApiInfo> = ignores
        .iter()
        .map(|i| ApiInfo {
            id: 0,
            path: i.path.clone(),
            description: "".to_string(),
            api_group: "".to_string(),
            method: i.method.clone(),
        })
        .collect();

    // 获取已注册的路由（排除被忽略的）
    let registered_routes = get_registered_routes();
    let cache_apis: Vec<RouteInfo> = registered_routes
        .into_iter()
        .filter(|r| {
            !ignores.iter().any(|i| i.path == r.path && i.method == r.method)
        })
        .collect();

    // 找出新增的 API（在路由中但不在数据库中）
    let new_apis: Vec<ApiInfo> = cache_apis
        .iter()
        .filter(|r| {
            !db_apis.iter().any(|a| a.path == r.path && a.method == r.method)
        })
        .map(|r| ApiInfo {
            id: 0,
            path: r.path.clone(),
            description: "".to_string(),
            api_group: "".to_string(),
            method: r.method.clone(),
        })
        .collect();

    // 找出需要删除的 API（在数据库中但不在路由中）
    let delete_apis: Vec<ApiInfo> = db_apis
        .iter()
        .filter(|a| {
            !cache_apis.iter().any(|r| r.path == a.path && r.method == a.method)
        })
        .map(|a| ApiInfo::from(a.clone()))
        .collect();

    Ok(SyncApiResult {
        new_apis,
        delete_apis,
        ignore_apis,
    })
}

/// 忽略 API，对应 Gin-Vue-Admin 的 apiService.IgnoreApi()
pub async fn ignore_api(db: &DatabaseConnection, path: String, method: String, flag: bool) -> Result<()> {
    if flag {
        // 添加忽略
        let model = sys_ignore_api::ActiveModel {
            path: Set(path),
            method: Set(method),
            ..Default::default()
        };
        model.insert(db).await?;
    } else {
        // 取消忽略
        sys_ignore_api::Entity::delete_many()
            .filter(sys_ignore_api::Column::Path.eq(&path))
            .filter(sys_ignore_api::Column::Method.eq(&method))
            .exec(db)
            .await?;
    }
    Ok(())
}

/// SyncApis 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncApisReq {
    #[serde(default)]
    pub new_apis: Vec<ApiInfo>,
    #[serde(default)]
    pub delete_apis: Vec<ApiInfo>,
}

/// 确认同步 API，对应 Gin-Vue-Admin 的 apiService.EnterSyncApi()
pub async fn enter_sync_api(db: &DatabaseConnection, sync_apis: SyncApisReq) -> Result<()> {
    use sea_orm::TransactionTrait;
    let txn = db.begin().await?;

    // 创建新 API
    for api in &sync_apis.new_apis {
        let model = sys_api::ActiveModel {
            path: Set(api.path.clone()),
            description: Set(api.description.clone()),
            api_group: Set(api.api_group.clone()),
            method: Set(api.method.clone()),
            ..Default::default()
        };
        model.insert(&txn).await?;
    }

    // 删除旧 API（同时清除 Casbin 策略）
    for api in &sync_apis.delete_apis {
        // 删除 Casbin 策略
        crate::model::system::casbin_rule::Entity::delete_many()
            .filter(crate::model::system::casbin_rule::Column::V1.eq(&api.path))
            .filter(crate::model::system::casbin_rule::Column::V2.eq(&api.method))
            .exec(&txn)
            .await?;

        // 删除 API
        sys_api::Entity::delete_many()
            .filter(sys_api::Column::Path.eq(&api.path))
            .filter(sys_api::Column::Method.eq(&api.method))
            .exec(&txn)
            .await?;
    }

    txn.commit().await?;
    Ok(())
}
