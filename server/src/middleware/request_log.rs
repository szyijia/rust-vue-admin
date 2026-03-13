use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use std::time::Instant;
use tracing::info;

/// 请求日志中间件
/// 记录：请求方法、请求路径、客户端IP、响应状态码、耗时、响应体内容
///
/// 响应体内容规则：
/// - 如果响应体是可见的文本字符串，则显示（最多前1024字节）
/// - 如果内容超过1024字节，显示前1024字节并标注截断
/// - 非可见字符/二进制内容不显示
pub async fn request_log(req: Request, next: Next) -> Response {
    let start = Instant::now();

    // 提取请求信息
    let method = req.method().clone();
    let uri = req.uri().path().to_string();
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();
    let full_path = format!("{}{}", uri, query);

    // 提取客户端 IP
    let client_ip = extract_client_ip(&req);

    // 执行后续处理链
    let response = next.run(req).await;

    // 获取响应状态码
    let status = response.status().as_u16();

    // 拆分响应，读取响应体
    let (parts, body) = response.into_parts();
    let mut stream = body.into_data_stream();
    let mut body_buf = BytesMut::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(data) => body_buf.extend_from_slice(&data),
            Err(_) => break,
        }
    }
    let body_bytes: Bytes = body_buf.freeze();

    // 计算耗时
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;

    // 处理响应体内容用于日志显示
    let body_display = format_response_body(&body_bytes);

    // 打印请求日志
    if body_display.is_empty() {
        info!(
            "[请求日志] {} {} | IP: {} | 状态: {} | 耗时: {:.2}ms",
            method, full_path, client_ip, status, duration_ms
        );
    } else {
        info!(
            "[请求日志] {} {} | IP: {} | 状态: {} | 耗时: {:.2}ms | 响应: {}",
            method, full_path, client_ip, status, duration_ms, body_display
        );
    }

    // 重新构建响应体并返回
    Response::from_parts(parts, Body::from(body_bytes))
}

/// 提取客户端 IP 地址
/// 优先级：X-Forwarded-For > X-Real-IP > 连接地址
fn extract_client_ip(req: &Request) -> String {
    // 1. 优先从 X-Forwarded-For 获取（可能包含多个IP，取第一个）
    if let Some(forwarded_for) = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(first_ip) = forwarded_for.split(',').next() {
            let ip = first_ip.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }

    // 2. 从 X-Real-IP 获取
    if let Some(real_ip) = req
        .headers()
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
    {
        let ip = real_ip.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }

    // 3. 从连接信息获取
    if let Some(connect_info) = req
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
    {
        return connect_info.0.ip().to_string();
    }

    "unknown".to_string()
}

/// 格式化响应体用于日志显示
///
/// - 如果是可见的 UTF-8 文本，显示内容（最多前1024字节）
/// - 如果超过1024字节，截断并标注
/// - 如果包含非可见字符或不是有效 UTF-8，返回空字符串（不显示）
fn format_response_body(body: &Bytes) -> String {
    const MAX_DISPLAY_BYTES: usize = 1024;

    if body.is_empty() {
        return String::new();
    }

    // 取前 MAX_DISPLAY_BYTES 字节用于检查和显示
    let truncated = body.len() > MAX_DISPLAY_BYTES;
    let check_bytes = if truncated {
        &body[..MAX_DISPLAY_BYTES]
    } else {
        &body[..]
    };

    // 尝试转为 UTF-8 字符串
    // 如果截断导致 UTF-8 字符被截断，尝试找到最后一个完整字符的边界
    let text = if truncated {
        match std::str::from_utf8(check_bytes) {
            Ok(s) => s.to_string(),
            Err(e) => {
                // 截断可能切割了多字节UTF-8字符，取有效部分
                let valid_up_to = e.valid_up_to();
                if valid_up_to == 0 {
                    return String::new();
                }
                match std::str::from_utf8(&check_bytes[..valid_up_to]) {
                    Ok(s) => s.to_string(),
                    Err(_) => return String::new(),
                }
            }
        }
    } else {
        match std::str::from_utf8(check_bytes) {
            Ok(s) => s.to_string(),
            Err(_) => return String::new(),
        }
    };

    // 检查是否包含非可见字符（允许常见的空白字符：空格、换行、回车、制表符）
    if text.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
        return String::new();
    }

    if truncated {
        format!("{}...(共{}字节，已截断)", text, body.len())
    } else {
        text
    }
}
