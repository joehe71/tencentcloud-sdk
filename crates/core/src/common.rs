use std::borrow::Cow;

use serde::{Serialize, de::DeserializeOwned};

use crate::{ApiResponse, Client, Operation, RequestOptions, SdkError};

/// 持有固定产品和版本的通用入口，不依赖产品模型。
#[derive(Clone)]
pub struct CommonClient {
    client: Client,
    service: Cow<'static, str>,
    version: Cow<'static, str>,
}

impl CommonClient {
    /// 绑定产品与 API 版本，共享 Client 的资源。
    ///
    /// # Errors
    /// 产品或版本无效时返回配置错误。
    pub fn new(
        client: Client,
        service: impl Into<Cow<'static, str>>,
        version: impl Into<Cow<'static, str>>,
    ) -> Result<Self, SdkError> {
        let service = service.into();
        let version = version.into();
        Operation::new(service.clone(), version.clone(), "Validate")?;
        Ok(Self {
            client,
            service,
            version,
        })
    }

    /// 使用默认选项发起一次 POST JSON 调用。
    ///
    /// # Errors
    /// 参数、凭证、签名、传输或腾讯云响应错误时返回对应分类。
    pub async fn call<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        action: &str,
        input: &I,
    ) -> Result<ApiResponse<O>, SdkError> {
        self.call_with_options(action, input, &RequestOptions::default())
            .await
    }

    /// 使用单次请求选项调用，不修改共享 Client。
    ///
    /// # Errors
    /// 参数、凭证、签名、传输或腾讯云响应错误时返回对应分类。
    pub async fn call_with_options<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        action: &str,
        input: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<O>, SdkError> {
        let operation = Operation::new(
            self.service.clone(),
            self.version.clone(),
            action.to_owned(),
        )?;
        self.client
            .execute_with_options(&operation, input, options)
            .await
    }
}
