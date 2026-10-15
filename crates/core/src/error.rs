use std::{error::Error, fmt};

use http::StatusCode;

/// 腾讯云返回的业务错误。消息仅通过显式访问取得，不进入默认格式化输出。
#[derive(Clone)]
#[non_exhaustive]
pub struct ServiceError {
    /// 官方错误码，不作重新映射。
    pub code: String,
    /// 官方错误消息，可能包含业务数据。
    pub message: String,
    /// 腾讯云请求标识。
    pub request_id: Option<String>,
    /// HTTP 状态码。
    pub status: StatusCode,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "腾讯云服务错误：{}（HTTP {}）", self.code, self.status)
    }
}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ServiceError {}

/// 保留分类和来源的 SDK 错误；默认格式化不包含响应体或凭证。
#[non_exhaustive]
pub enum SdkError {
    /// 客户端配置错误。
    Configuration(&'static str),
    /// 凭证缺失或无效。
    Credentials(&'static str),
    /// 签名输入或计算错误。
    Signing(&'static str),
    /// 请求无法编码为 JSON。
    Serialization(serde_json::Error),
    /// 底层传输错误，来源仅通过 `Error::source` 显式访问。
    Transport(Box<dyn Error + Send + Sync>),
    /// 调用超过 deadline，或底层连接超时。
    Timeout,
    /// 请求或响应体超过配置上限。
    BodyTooLarge {
        /// 允许的最大字节数。
        limit: usize,
    },
    /// 响应不符合腾讯云 JSON 协议。
    Protocol {
        /// 不包含响应原文的描述。
        message: &'static str,
        /// 收到的 HTTP 状态码。
        status: StatusCode,
        /// 从响应中取得的请求标识。
        request_id: Option<String>,
    },
    /// 腾讯云业务错误。
    Service(ServiceError),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => write!(f, "客户端配置错误：{message}"),
            Self::Credentials(message) => write!(f, "凭证错误：{message}"),
            Self::Signing(message) => write!(f, "签名错误：{message}"),
            Self::Serialization(_) => f.write_str("请求 JSON 编码失败"),
            Self::Transport(_) => f.write_str("HTTP 传输失败"),
            Self::Timeout => f.write_str("调用超时"),
            Self::BodyTooLarge { limit } => write!(f, "请求或响应体超过 {limit} 字节"),
            Self::Protocol {
                message, status, ..
            } => {
                write!(f, "响应协议错误：{message}（HTTP {status}）")
            }
            Self::Service(error) => fmt::Display::fmt(error, f),
        }
    }
}

impl fmt::Debug for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for SdkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            Self::Transport(error) => Some(error.as_ref()),
            Self::Service(error) => Some(error),
            _ => None,
        }
    }
}
