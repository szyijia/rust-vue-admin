// 错误日志服务
use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::model::system::sys_error;

/// 创建错误日志
pub async fn create_sys_error(
    db: &DatabaseConnection,
    form: Option<String>,
    info: Option<String>,
    level: Option<String>,
) -> Result<()> {
    let now = chrono::Local::now().naive_local();
    let model = sys_error::ActiveModel {
        form: Set(form),
        info: Set(info),
        level: Set(level),
        solution: Set(None),
        status: Set(Some("未处理".to_string())),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}
