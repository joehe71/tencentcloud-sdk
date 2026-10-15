use http::StatusCode;

/// 公共响应元数据，不额外保存完整响应体。
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ResponseMetadata {
    /// 腾讯云 RequestId，优先来自 JSON，缺失时使用响应 Header。
    pub request_id: Option<String>,
    /// HTTP 状态码。
    pub status: StatusCode,
}

/// 拥有数据所有权的强类型响应。
#[derive(Debug)]
#[non_exhaustive]
pub struct ApiResponse<T> {
    /// 解析后的 Response 业务对象。
    pub data: T,
    /// 统一元数据。
    pub metadata: ResponseMetadata,
}
