use std::time::Duration;

use futures_util::TryStreamExt;
use reqwest::{Client, redirect::Policy, retry};

use super::{Transport, TransportRequest, TransportResponse};
use crate::{BoxFuture, SdkError};

/// 共享 reqwest 连接池的默认实现。关闭重定向、自动重试和环境代理。
#[derive(Clone)]
pub struct ReqwestTransport(Client);

impl ReqwestTransport {
    /// 使用 Rustls 创建连接池；连接超时必须非零。
    ///
    /// # Errors
    /// 超时配置无效或 TLS/HTTP 客户端初始化失败时返回错误。
    pub fn new(connect_timeout: Duration) -> Result<Self, SdkError> {
        if connect_timeout.is_zero() {
            return Err(SdkError::Configuration("连接超时必须大于零"));
        }
        let client = Client::builder()
            .connect_timeout(connect_timeout)
            .redirect(Policy::none())
            .retry(retry::never())
            .no_proxy()
            .build()
            .map_err(map_error)?;
        Ok(Self(client))
    }
}

fn map_error(error: reqwest::Error) -> SdkError {
    if error.is_timeout() {
        SdkError::Timeout
    } else {
        SdkError::Transport(Box::new(error.without_url()))
    }
}

impl Transport for ReqwestTransport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        Box::pin(async move {
            let response = self
                .0
                .request(request.method, request.uri.to_string())
                .headers(request.headers)
                .body(request.body)
                .send()
                .await
                .map_err(map_error)?;
            Ok(TransportResponse {
                status: response.status(),
                headers: response.headers().clone(),
                body: Box::pin(response.bytes_stream().map_err(map_error)),
            })
        })
    }
}
