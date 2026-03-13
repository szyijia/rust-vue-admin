// 邮件 API - 对应 Gin-Vue-Admin server/plugin/email/api/sys_email.go
use axum::extract::{Extension, State};
use axum::Json;
use tracing::error;

use crate::global::{ApiResponse, AppState};
use crate::utils::Claims;

/// POST /email/emailTest
/// 发送测试邮件（对应 Go EmailTest）
pub async fn email_test(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Json<ApiResponse<()>> {
    let config = state.get_config();
    let email_config = &config.email;

    // 使用 lettre 发送测试邮件
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    let from = match email_config.from.parse() {
        Ok(f) => f,
        Err(e) => {
            error!("邮件发送地址格式错误: {}", e);
            return Json(ApiResponse::fail(7001, format!("发送失败: 邮件发送地址格式错误 {}", e), ()));
        }
    };
    let to = match email_config.to.parse() {
        Ok(t) => t,
        Err(e) => {
            error!("邮件接收地址格式错误: {}", e);
            return Json(ApiResponse::fail(7001, format!("发送失败: 邮件接收地址格式错误 {}", e), ()));
        }
    };

    let email = match Message::builder()
        .from(if !email_config.nickname.is_empty() {
            format!("{} <{}>", email_config.nickname, email_config.from).parse().unwrap_or(from)
        } else {
            from
        })
        .to(to)
        .subject("test")
        .header(ContentType::TEXT_HTML)
        .body("test".to_string())
    {
        Ok(e) => e,
        Err(e) => {
            error!("构建邮件失败: {}", e);
            return Json(ApiResponse::fail(7001, format!("发送失败: {}", e), ()));
        }
    };

    let creds = Credentials::new(email_config.from.clone(), email_config.secret.clone());

    let mailer_result = if email_config.is_ssl {
        SmtpTransport::relay(&email_config.host)
            .map(|builder| {
                builder
                    .credentials(creds)
                    .port(email_config.port)
                    .build()
            })
    } else {
        SmtpTransport::builder_dangerous(&email_config.host)
            .port(email_config.port)
            .credentials(creds);
        Ok(SmtpTransport::builder_dangerous(&email_config.host)
            .port(email_config.port)
            .credentials(Credentials::new(email_config.from.clone(), email_config.secret.clone()))
            .build())
    };

    match mailer_result {
        Ok(mailer) => {
            match mailer.send(&email) {
                Ok(_) => Json(ApiResponse::ok_msg("发送成功")),
                Err(e) => {
                    error!("发送测试邮件失败: {}", e);
                    Json(ApiResponse::fail(7001, format!("发送失败: {}", e), ()))
                }
            }
        }
        Err(e) => {
            error!("创建邮件传输器失败: {}", e);
            Json(ApiResponse::fail(7001, format!("发送失败: {}", e), ()))
        }
    }
}
