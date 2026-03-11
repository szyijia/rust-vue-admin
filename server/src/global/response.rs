use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// 统一 API 响应结构，对应 Gin-Vue-Admin 的 response.Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    /// 状态码：0 表示成功，非 0 表示失败
    pub code: i32,
    /// 响应数据
    pub data: T,
    /// 响应消息
    pub msg: String,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            data,
            msg: "操作成功".to_string(),
        }
    }

    /// 成功响应（自定义消息）
    pub fn ok_with_data(data: T, msg: impl Into<String>) -> Self {
        Self {
            code: 0,
            data,
            msg: msg.into(),
        }
    }

    /// 失败响应
    pub fn fail(code: i32, msg: impl Into<String>, data: T) -> Self {
        Self {
            code,
            data,
            msg: msg.into(),
        }
    }

    /// 错误响应（data 使用 Default 值，适合所有实现了 Default 的类型）
    pub fn err_default(code: i32, msg: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            code,
            data: T::default(),
            msg: msg.into(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// 针对 () 类型的快捷错误响应方法（用于中间件返回错误）
impl ApiResponse<()> {
    /// 成功响应（仅消息，无数据）
    pub fn ok_msg(msg: impl Into<String>) -> Self {
        Self {
            code: 0,
            data: (),
            msg: msg.into(),
        }
    }

    /// 失败响应（2参数版本，data 为 ()）
    pub fn fail_msg(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            data: (),
            msg: msg.into(),
        }
    }

    /// 401 未认证
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self {
            code: 401,
            data: (),
            msg: msg.into(),
        }
    }

    /// 403 无权限
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self {
            code: 403,
            data: (),
            msg: msg.into(),
        }
    }

    /// 429 请求过于频繁
    pub fn too_many_requests(msg: impl Into<String>) -> Self {
        Self {
            code: 429,
            data: (),
            msg: msg.into(),
        }
    }
}

impl ApiResponse<()> {
    /// 将错误响应转换为带正确 HTTP 状态码的 axum Response
    /// 中间件使用此方法返回错误
    pub fn into_http_response(self) -> Response {
        let status = match self.code {
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            429 => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::OK,
        };
        (status, Json(self)).into_response()
    }
}

/// 分页结果
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T: Serialize> {
    /// 数据列表
    pub list: Vec<T>,
    /// 总条数
    pub total: u64,
    /// 当前页
    pub page: i64,
    /// 每页条数
    pub page_size: i64,
}

impl<T: Serialize> PageResult<T> {
    pub fn new(list: Vec<T>, total: u64, page: i64, page_size: i64) -> Self {
        Self {
            list,
            total,
            page,
            page_size,
        }
    }
}

/// 空数据响应（用于无返回值的接口）
pub type EmptyResponse = ApiResponse<serde_json::Value>;

impl EmptyResponse {
    pub fn success() -> Self {
        ApiResponse::ok(serde_json::Value::Null)
    }

    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        ApiResponse::fail(code, msg, serde_json::Value::Null)
    }
}
