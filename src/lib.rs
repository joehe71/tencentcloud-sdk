//! 腾讯云 SDK 的统一入口。默认不启用产品或 HTTP 实现。
//!
//! 基础类型由 core 重导出；`transport-reqwest` 启用共享连接池的默认传输实现。

pub use tencentcloud_sdk_core::{
    ApiResponse, BoxFuture, Client, ClientBuilder, CommonClient, Credentials, Endpoint, Language,
    Operation, RequestOptions, ResponseMetadata, SdkError, ServiceError,
};
pub use tencentcloud_sdk_core::{credentials, runtime, transport};
