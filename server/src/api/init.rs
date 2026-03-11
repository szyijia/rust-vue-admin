use axum::extract::State;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::{
    config::AppConfig,
    global::{ApiResponse, AppState},
    initialize,
    migration::Migrator,
    utils::hash_password,
};
use sea_orm_migration::MigratorTrait;

// ===== 请求/响应 DTO =====

/// 初始化数据库请求，对应 Gin-Vue-Admin 的 request.InitDB
#[derive(Debug, Deserialize)]
pub struct InitDbRequest {
    /// admin 账号的初始密码
    pub admin_password: String,
    /// 数据库类型: mysql / pgsql / sqlite
    #[serde(default = "default_db_type")]
    pub db_type: String,
    /// 数据库服务器地址（sqlite 不需要）
    #[serde(default)]
    pub host: String,
    /// 数据库端口（sqlite 不需要）
    #[serde(default)]
    pub port: String,
    /// 数据库用户名（sqlite 不需要）
    #[serde(default)]
    pub user_name: String,
    /// 数据库密码（sqlite 不需要）
    #[serde(default)]
    pub password: String,
    /// 数据库名称
    pub db_name: String,
    /// sqlite 文件路径（仅 sqlite 使用）
    #[serde(default)]
    pub db_path: String,
}

fn default_db_type() -> String { "sqlite".to_string() }

#[derive(Debug, Serialize)]
pub struct CheckDbResponse {
    pub need_init: bool,
}

// ===== Handler =====

/// GET /init/checkdb
/// 检查数据库是否已初始化，对应 Gin-Vue-Admin 的 dbApi.CheckDB()
pub async fn check_db(
    State(state): State<AppState>,
) -> ApiResponse<CheckDbResponse> {
    let need_init = !state.has_db().await;
    let msg = if need_init { "前往初始化数据库" } else { "数据库无需初始化" };
    ApiResponse::ok_with_data(CheckDbResponse { need_init }, msg)
}

/// POST /init/initdb
/// 初始化数据库，对应 Gin-Vue-Admin 的 dbApi.InitDB()
/// 流程：1.验证参数 → 2.连接数据库 → 3.执行迁移 → 4.创建 admin 用户 → 5.写回配置 → 6.初始化 Casbin
pub async fn init_db(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<InitDbRequest>,
) -> ApiResponse<serde_json::Value> {
    // 1. 检查是否已初始化
    if state.has_db().await {
        return ApiResponse::fail(7010, "数据库已初始化，无需重复操作", serde_json::Value::Null);
    }

    // 2. 参数校验
    if req.admin_password.len() < 6 {
        return ApiResponse::fail(7002, "管理员密码至少6位", serde_json::Value::Null);
    }
    if req.db_name.is_empty() {
        return ApiResponse::fail(7002, "数据库名称不能为空", serde_json::Value::Null);
    }

    // 3. 构建临时配置用于连接数据库（克隆当前配置并修改数据库相关字段）
    let mut temp_config: AppConfig = (*state.config).clone();
    let db_type = req.db_type.to_lowercase();
    temp_config.system.db_type = db_type.clone();
    temp_config.system.disable_auto_migrate = true; // initdb 自己管理迁移

    match db_type.as_str() {
        "mysql" => {
            let host = if req.host.is_empty() { "127.0.0.1".to_string() } else { req.host.clone() };
            let port = if req.port.is_empty() { "3306".to_string() } else { req.port.clone() };
            temp_config.mysql.path = host;
            temp_config.mysql.port = port;
            temp_config.mysql.username = req.user_name.clone();
            temp_config.mysql.password = req.password.clone();
            temp_config.mysql.db_name = req.db_name.clone();
            temp_config.mysql.config = "charset=utf8mb4&parseTime=True&loc=Local".to_string();
        }
        "pgsql" | "postgres" | "postgresql" => {
            let host = if req.host.is_empty() { "127.0.0.1".to_string() } else { req.host.clone() };
            let port = if req.port.is_empty() { "5432".to_string() } else { req.port.clone() };
            temp_config.pgsql.path = host;
            temp_config.pgsql.port = port;
            temp_config.pgsql.username = req.user_name.clone();
            temp_config.pgsql.password = req.password.clone();
            temp_config.pgsql.db_name = req.db_name.clone();
            temp_config.pgsql.config = "sslmode=disable TimeZone=Asia/Shanghai".to_string();
            temp_config.system.db_type = "pgsql".to_string();
        }
        "sqlite" => {
            // sqlite 路径：如果 db_path 为空，使用当前目录
            let path = if req.db_path.is_empty() {
                format!("{}.db", req.db_name)
            } else if req.db_path.ends_with('/') || req.db_path.ends_with('\\') {
                format!("{}{}.db", req.db_path, req.db_name)
            } else {
                format!("{}/{}.db", req.db_path, req.db_name)
            };
            temp_config.sqlite.path = path;
        }
        other => {
            return ApiResponse::fail(7002, format!("不支持的数据库类型: {}，请使用 mysql/pgsql/sqlite", other), serde_json::Value::Null);
        }
    }

    // 4. 连接数据库
    warn!("⚠️  [初始化] 用户正在通过 /init/initdb 接口初始化数据库 (type={}, db={})", db_type, req.db_name);
    info!("正在连接数据库 (type={}, db={})...", db_type, req.db_name);
    let db = match initialize::init_db(&temp_config).await {
        Ok(db) => db,
        Err(e) => {
            error!("数据库连接失败: {}", e);
            return ApiResponse::fail(7011, format!("数据库连接失败: {}", e), serde_json::Value::Null);
        }
    };

    // 5. 执行数据库迁移（仅增量建表，不删除已有表）
    info!("正在执行数据库迁移（增量模式，已有表将被跳过）...");

    // 迁移前获取状态报告
    let mut migration_report: Vec<serde_json::Value> = Vec::new();
    let pre_status = Migrator::get_migration_with_status(&db).await;
    let pending_names: std::collections::HashSet<String> = match &pre_status {
        Ok(migrations) => {
            info!("📋 数据库迁移状态报告：");
            for m in migrations {
                let (status_str, status_icon) = match m.status() {
                    sea_orm_migration::MigrationStatus::Applied => ("已存在", "✅"),
                    sea_orm_migration::MigrationStatus::Pending => ("待执行", "⏳"),
                };
                info!("  {} {} - {}", status_icon, status_str, m.name());
            }
            migrations.iter()
                .filter(|m| m.status() == sea_orm_migration::MigrationStatus::Pending)
                .map(|m| m.name().to_string())
                .collect()
        }
        Err(e) => {
            warn!("⚠️  获取迁移状态失败: {}，将尝试直接执行迁移", e);
            std::collections::HashSet::new()
        }
    };

    // 执行增量迁移（只执行未运行过的迁移，表已存在会被 IF NOT EXISTS 跳过）
    if let Err(e) = Migrator::up(&db, None).await {
        error!("数据库迁移失败: {}", e);
        return ApiResponse::fail(7012, format!("数据库迁移失败: {}（已存在的表不受影响，请检查数据库状态）", e), serde_json::Value::Null);
    }

    // 迁移后获取最终状态，生成报告
    match Migrator::get_migration_with_status(&db).await {
        Ok(migrations) => {
            for m in &migrations {
                let was_pending = pending_names.contains(m.name());
                let status = match m.status() {
                    sea_orm_migration::MigrationStatus::Applied if was_pending => "新建成功",
                    sea_orm_migration::MigrationStatus::Applied => "已存在（跳过）",
                    sea_orm_migration::MigrationStatus::Pending => "执行失败",
                };
                migration_report.push(serde_json::json!({
                    "name": m.name(),
                    "status": status,
                }));
                info!("  📌 {} - {}", status, m.name());
            }
        }
        Err(e) => warn!("⚠️  获取迁移后状态失败: {}", e),
    }
    info!("✅ 数据库迁移完成");

    // 6. 创建 admin 用户（使用用户指定的密码）
    let hashed_password = match hash_password(&req.admin_password) {
        Ok(h) => h,
        Err(e) => {
            error!("密码哈希失败: {}", e);
            return ApiResponse::fail(7013, format!("密码处理失败: {}", e), serde_json::Value::Null);
        }
    };

    if let Err(e) = create_admin_user(&db, &hashed_password).await {
        error!("创建 admin 用户失败: {}", e);
        return ApiResponse::fail(7014, format!("创建管理员账号失败: {}", e), serde_json::Value::Null);
    }
    info!("✅ admin 用户创建成功");

    // 7. 将数据库连接设置到全局状态
    state.set_db(db.clone()).await;

    // 8. 初始化 Casbin
    match initialize::init_casbin(&db).await {
        Ok(enforcer) => {
            state.set_enforcer(enforcer).await;
            info!("✅ Casbin 权限引擎初始化完成");
        }
        Err(e) => {
            error!("Casbin 初始化失败: {}（不影响运行）", e);
        }
    }

    // 9. 写回配置文件（持久化数据库配置，下次启动自动连接）
    if let Err(e) = write_config_to_file(&temp_config).await {
        error!("写回配置文件失败: {}（不影响运行）", e);
    } else {
        info!("✅ 配置已写回 config.yaml");
    }

    info!("🎉 [初始化] 数据库初始化全部完成，配置已持久化。下次启动将自动连接数据库并执行增量迁移(migrate 模式)");
    warn!("⚠️  [初始化] 请牢记 admin 密码！如需重新初始化，需先清空 config.yaml 中的数据库配置并重启服务");

    ApiResponse::ok(serde_json::json!({
        "msg": "数据库初始化成功，admin 账号已创建，请使用设定的密码登录",
        "migrations": migration_report
    }))
}

/// 创建 admin 超级管理员用户
async fn create_admin_user(
    db: &sea_orm::DatabaseConnection,
    hashed_password: &str,
) -> anyhow::Result<()> {
    use sea_orm::{ActiveModelTrait, Set};
    use crate::model::system::sys_user;
    use uuid::Uuid;

    let admin = sys_user::ActiveModel {
        uuid: Set(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()),
        username: Set("admin".to_string()),
        password: Set(hashed_password.to_string()),
        nick_name: Set("超级管理员".to_string()),
        header_img: Set(String::new()),
        phone: Set(String::new()),
        email: Set(String::new()),
        enable: Set(1),
        authority_id: Set(888i64),
        ..Default::default()
    };

    admin.insert(db).await?;
    Ok(())
}

/// 将数据库配置写回 config.yaml，下次启动时自动连接
async fn write_config_to_file(config: &AppConfig) -> anyhow::Result<()> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let content = std::fs::read_to_string(&config_path)?;

    // 使用 serde_yaml 解析并更新
    let mut yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;

    // 更新 system 节
    // 初始化完成后：
    //   - disable_auto_migrate = false → 下次启动自动执行增量迁移（安全，只建表不删表）
    if let Some(system) = yaml_value.get_mut("system") {
        if let Some(obj) = system.as_mapping_mut() {
            obj.insert(
                serde_yaml::Value::String("db_type".to_string()),
                serde_yaml::Value::String(config.system.db_type.clone()),
            );
            obj.insert(
                serde_yaml::Value::String("disable_auto_migrate".to_string()),
                serde_yaml::Value::Bool(false),
            );
        }
    }

    // 根据 db_type 更新对应数据库配置节
    match config.system.db_type.as_str() {
        "mysql" => {
            if let Some(mysql) = yaml_value.get_mut("mysql") {
                if let Some(obj) = mysql.as_mapping_mut() {
                    obj.insert(serde_yaml::Value::String("path".to_string()), serde_yaml::Value::String(config.mysql.path.clone()));
                    obj.insert(serde_yaml::Value::String("port".to_string()), serde_yaml::Value::String(config.mysql.port.clone()));
                    obj.insert(serde_yaml::Value::String("db_name".to_string()), serde_yaml::Value::String(config.mysql.db_name.clone()));
                    obj.insert(serde_yaml::Value::String("username".to_string()), serde_yaml::Value::String(config.mysql.username.clone()));
                    obj.insert(serde_yaml::Value::String("password".to_string()), serde_yaml::Value::String(config.mysql.password.clone()));
                }
            }
        }
        "pgsql" => {
            if let Some(pgsql) = yaml_value.get_mut("pgsql") {
                if let Some(obj) = pgsql.as_mapping_mut() {
                    obj.insert(serde_yaml::Value::String("path".to_string()), serde_yaml::Value::String(config.pgsql.path.clone()));
                    obj.insert(serde_yaml::Value::String("port".to_string()), serde_yaml::Value::String(config.pgsql.port.clone()));
                    obj.insert(serde_yaml::Value::String("db_name".to_string()), serde_yaml::Value::String(config.pgsql.db_name.clone()));
                    obj.insert(serde_yaml::Value::String("username".to_string()), serde_yaml::Value::String(config.pgsql.username.clone()));
                    obj.insert(serde_yaml::Value::String("password".to_string()), serde_yaml::Value::String(config.pgsql.password.clone()));
                }
            }
        }
        "sqlite" => {
            if let Some(sqlite) = yaml_value.get_mut("sqlite") {
                if let Some(obj) = sqlite.as_mapping_mut() {
                    obj.insert(serde_yaml::Value::String("path".to_string()), serde_yaml::Value::String(config.sqlite.path.clone()));
                }
            }
        }
        _ => {}
    }

    let new_content = serde_yaml::to_string(&yaml_value)?;
    std::fs::write(&config_path, new_content)?;

    Ok(())
}
