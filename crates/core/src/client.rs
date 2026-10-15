use std::{collections::BTreeMap, sync::Arc, time::Duration};

use bytes::Bytes;
use http::{
    HeaderValue, Method,
    header::{AUTHORIZATION, CONTENT_TYPE, HOST},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::time::{Instant, timeout_at};

use crate::{
    ApiResponse, Credentials, Endpoint, Operation, RequestOptions, SdkError,
    credentials::{EnvironmentCredentialsProvider, ProvideCredentials, StaticCredentialsProvider},
    operation::{validate_region, validate_service},
    protocol::{self, RawResponse},
    runtime::{Clock, SystemClock},
    signing,
    transport::{Transport, TransportRequest},
};

const DEFAULT_BODY_LIMIT: usize = 32 * 1024 * 1024;

/// 轻量可克隆的共享客户端，构建后配置不可变，不保存每请求的可变状态。
#[derive(Clone)]
pub struct Client(Arc<ClientInner>);

struct ClientInner {
    credentials: Arc<dyn ProvideCredentials>,
    transport: Arc<dyn Transport>,
    clock: Arc<dyn Clock>,
    region: Option<String>,
    endpoints: BTreeMap<String, Endpoint>,
    timeout: Duration,
    request_limit: usize,
    response_limit: usize,
}

impl Client {
    /// 创建配置构造器。不会访问环境变量或网络。
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// 按操作描述调用一次，不进行自动重试。
    ///
    /// # Errors
    /// 配置、编码、凭证、签名、网络、超时或响应错误时返回对应分类。
    pub async fn execute<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        operation: &Operation,
        input: &I,
    ) -> Result<ApiResponse<O>, SdkError> {
        self.execute_with_options(operation, input, &RequestOptions::default())
            .await
    }

    /// 以局部选项执行一次；需要启用时间驱动的 Tokio Runtime。
    ///
    /// # Errors
    /// 配置、编码、凭证、签名、网络、超时或响应错误时返回对应分类。
    pub async fn execute_with_options<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        operation: &Operation,
        input: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<O>, SdkError> {
        let budget = options.timeout.unwrap_or(self.0.timeout);
        if budget.is_zero() {
            return Err(SdkError::Configuration("调用超时必须大于零"));
        }
        let deadline = Instant::now()
            .checked_add(budget)
            .ok_or(SdkError::Configuration("调用超时超出有效范围"))?;
        timeout_at(deadline, async {
            let body = protocol::encode(input, self.0.request_limit)?;
            check_deadline(deadline)?;
            let raw = self.execute_raw(operation, body, options, deadline).await?;
            check_deadline(deadline)?;
            let response = protocol::decode(&raw)?;
            check_deadline(deadline)?;
            Ok(response)
        })
        .await
        .map_err(|_| SdkError::Timeout)?
    }

    // 字节级执行链路不随每个请求/响应类型重复单态化。
    async fn execute_raw(
        &self,
        operation: &Operation,
        body: Bytes,
        options: &RequestOptions,
        deadline: Instant,
    ) -> Result<RawResponse, SdkError> {
        let region = options.region.as_deref().or(self.0.region.as_deref());
        if let Some(region) = region {
            validate_region(region)?;
        }
        let endpoint = match self.0.endpoints.get(operation.service()) {
            Some(endpoint) => endpoint.clone(),
            None => Endpoint::for_service(operation.service())?,
        };
        let host = endpoint
            .uri()
            .authority()
            .ok_or(SdkError::Configuration("API 端点缺少主机"))?
            .as_str();
        let credentials = self.0.credentials.provide_credentials().await?;
        check_deadline(deadline)?;
        let timestamp = self.0.clock.unix_timestamp()?;
        let authorization = signing::authorization(
            &credentials,
            operation.service(),
            host,
            operation.action(),
            &body,
            timestamp,
        )?;
        let mut headers = options.headers.clone();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(signing::CONTENT_TYPE),
        );
        headers.insert(HOST, header_value(host)?);
        headers.insert(AUTHORIZATION, authorization);
        headers.insert("x-tc-action", header_value(operation.action())?);
        headers.insert("x-tc-version", header_value(operation.version())?);
        headers.insert("x-tc-timestamp", header_value(&timestamp.to_string())?);
        headers.insert(
            "x-tc-requestclient",
            HeaderValue::from_static(concat!("SDK_RUST_", env!("CARGO_PKG_VERSION"))),
        );
        if let Some(region) = region {
            headers.insert("x-tc-region", header_value(region)?);
        }
        if let Some(token) = credentials.token() {
            headers.insert("x-tc-token", token.clone());
        }
        if let Some(language) = options.language {
            headers.insert("x-tc-language", HeaderValue::from_static(language.as_str()));
        }
        check_deadline(deadline)?;
        let response = self
            .0
            .transport
            .send(TransportRequest {
                method: Method::POST,
                uri: endpoint.uri().clone(),
                headers,
                body,
            })
            .await?;
        protocol::collect(response, self.0.response_limit).await
    }
}

fn header_value(value: &str) -> Result<HeaderValue, SdkError> {
    HeaderValue::from_str(value).map_err(|_| SdkError::Configuration("公共 Header 值无效"))
}

fn check_deadline(deadline: Instant) -> Result<(), SdkError> {
    if Instant::now() >= deadline {
        Err(SdkError::Timeout)
    } else {
        Ok(())
    }
}

enum CredentialSource {
    Provider(Arc<dyn ProvideCredentials>),
    Environment,
}

/// 客户端构造器。默认无凭证；不启用传输 feature 时也没有默认 Transport。
pub struct ClientBuilder {
    credentials: Option<CredentialSource>,
    transport: Option<Arc<dyn Transport>>,
    clock: Arc<dyn Clock>,
    region: Option<String>,
    endpoints: BTreeMap<String, Endpoint>,
    timeout: Duration,
    request_limit: usize,
    response_limit: usize,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            credentials: None,
            transport: None,
            clock: Arc::new(SystemClock),
            region: None,
            endpoints: BTreeMap::new(),
            timeout: Duration::from_secs(60),
            request_limit: DEFAULT_BODY_LIMIT,
            response_limit: DEFAULT_BODY_LIMIT,
        }
    }
}

impl ClientBuilder {
    /// 使用调用方显式提供的静态凭证。
    #[must_use]
    pub fn credentials(self, credentials: Credentials) -> Self {
        self.credentials_provider(StaticCredentialsProvider::new(credentials))
    }
    /// 在 build 时读取环境凭证，不探测实例元数据或调用 STS。
    #[must_use]
    pub fn credentials_from_env(mut self) -> Self {
        self.credentials = Some(CredentialSource::Environment);
        self
    }
    /// 注入自定义凭证来源。
    #[must_use]
    pub fn credentials_provider(mut self, provider: impl ProvideCredentials + 'static) -> Self {
        self.credentials = Some(CredentialSource::Provider(Arc::new(provider)));
        self
    }
    /// 注入自定义传输。传输实例在所有 Client 克隆间共享。
    #[must_use]
    pub fn transport(mut self, transport: impl Transport + 'static) -> Self {
        self.transport = Some(Arc::new(transport));
        self
    }
    /// 注入可被多个独立 Client 共享的传输实例。
    #[must_use]
    pub fn shared_transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }
    /// 注入时间来源，例如固定时钟。
    #[must_use]
    pub fn clock(mut self, clock: impl Clock + 'static) -> Self {
        self.clock = Arc::new(clock);
        self
    }
    /// 设置默认 Region；不要求所有 API 都传 Region。
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    /// 为指定产品设置端点，不影响其他产品，也不改变业务 Region。
    #[must_use]
    pub fn endpoint(mut self, service: impl Into<String>, endpoint: Endpoint) -> Self {
        self.endpoints.insert(service.into(), endpoint);
        self
    }
    /// 默认调用预算，包含凭证、编码、发送及响应体读取。
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// 设置请求 JSON 的最大编码字节数。
    #[must_use]
    pub fn max_request_body(mut self, limit: usize) -> Self {
        self.request_limit = limit;
        self
    }
    /// 设置响应体的实际累计字节上限，不依赖 Content-Length。
    #[must_use]
    pub fn max_response_body(mut self, limit: usize) -> Self {
        self.response_limit = limit;
        self
    }

    /// 验证配置并创建 Client，不发送网络请求。
    ///
    /// # Errors
    /// 凭证、Region、超时或 body 限制无效，或没有可用传输时返回错误。
    pub fn build(self) -> Result<Client, SdkError> {
        if self.timeout.is_zero() || self.request_limit == 0 || self.response_limit == 0 {
            return Err(SdkError::Configuration("超时和 body 上限必须大于零"));
        }
        if let Some(region) = &self.region {
            validate_region(region)?;
        }
        for service in self.endpoints.keys() {
            validate_service(service)?;
        }
        let credentials = match self.credentials {
            Some(CredentialSource::Provider(provider)) => provider,
            Some(CredentialSource::Environment) => {
                Arc::new(EnvironmentCredentialsProvider::load()?)
            }
            None => return Err(SdkError::Configuration("必须显式配置凭证来源")),
        };
        let transport = match self.transport {
            Some(transport) => transport,
            None => default_transport()?,
        };
        Ok(Client(Arc::new(ClientInner {
            credentials,
            transport,
            clock: self.clock,
            region: self.region,
            endpoints: self.endpoints,
            timeout: self.timeout,
            request_limit: self.request_limit,
            response_limit: self.response_limit,
        })))
    }
}

#[cfg(feature = "transport-reqwest")]
fn default_transport() -> Result<Arc<dyn Transport>, SdkError> {
    use crate::transport::ReqwestTransport;
    Ok(Arc::new(ReqwestTransport::new(Duration::from_secs(10))?))
}

#[cfg(not(feature = "transport-reqwest"))]
fn default_transport() -> Result<Arc<dyn Transport>, SdkError> {
    Err(SdkError::Configuration(
        "请注入 Transport 或启用 transport-reqwest",
    ))
}
