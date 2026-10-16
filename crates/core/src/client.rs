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

/// A cheaply cloneable shared client with immutable configuration and no mutable per-request state.
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
    /// Creates a builder without reading environment variables or accessing the network.
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Executes an operation once without automatic retries.
    ///
    /// # Errors
    /// Returns the corresponding configuration, encoding, credential, signing, transport, timeout, or response error.
    pub async fn execute<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        operation: &Operation,
        input: &I,
    ) -> Result<ApiResponse<O>, SdkError> {
        self.execute_with_options(operation, input, &RequestOptions::default())
            .await
    }

    /// Executes once with per-call options; requires a Tokio runtime with its time driver enabled.
    ///
    /// # Errors
    /// Returns the corresponding configuration, encoding, credential, signing, transport, timeout, or response error.
    pub async fn execute_with_options<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        operation: &Operation,
        input: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<O>, SdkError> {
        let budget = options.timeout.unwrap_or(self.0.timeout);
        if budget.is_zero() {
            return Err(SdkError::Configuration(
                "call timeout must be greater than zero",
            ));
        }
        let deadline = Instant::now()
            .checked_add(budget)
            .ok_or(SdkError::Configuration("call timeout is out of range"))?;
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

    // Keep the byte-level execution path independent of request/response monomorphization.
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
            .ok_or(SdkError::Configuration("API endpoint has no host"))?
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
    HeaderValue::from_str(value).map_err(|_| SdkError::Configuration("invalid common header value"))
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

/// Builds a client with no default credentials or, without a transport feature, default transport.
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
    /// Uses static credentials explicitly supplied by the caller.
    #[must_use]
    pub fn credentials(self, credentials: Credentials) -> Self {
        self.credentials_provider(StaticCredentialsProvider::new(credentials))
    }
    /// Loads environment credentials at build time without probing instance metadata or calling STS.
    #[must_use]
    pub fn credentials_from_env(mut self) -> Self {
        self.credentials = Some(CredentialSource::Environment);
        self
    }
    /// Sets a custom credentials provider.
    #[must_use]
    pub fn credentials_provider(mut self, provider: impl ProvideCredentials + 'static) -> Self {
        self.credentials = Some(CredentialSource::Provider(Arc::new(provider)));
        self
    }
    /// Sets a custom transport shared by all clones of this client.
    #[must_use]
    pub fn transport(mut self, transport: impl Transport + 'static) -> Self {
        self.transport = Some(Arc::new(transport));
        self
    }
    /// Sets a transport instance that can be shared by independently built clients.
    #[must_use]
    pub fn shared_transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }
    /// Sets a time source, such as a fixed clock for tests.
    #[must_use]
    pub fn clock(mut self, clock: impl Clock + 'static) -> Self {
        self.clock = Arc::new(clock);
        self
    }
    /// Sets the default region; not every API requires one.
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    /// Overrides one service endpoint without affecting other services or the business region.
    #[must_use]
    pub fn endpoint(mut self, service: impl Into<String>, endpoint: Endpoint) -> Self {
        self.endpoints.insert(service.into(), endpoint);
        self
    }
    /// Sets the default call budget, including credentials, encoding, sending, and body collection.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Sets the maximum encoded JSON request size in bytes.
    #[must_use]
    pub fn max_request_body(mut self, limit: usize) -> Self {
        self.request_limit = limit;
        self
    }
    /// Limits actual accumulated response bytes, independently of Content-Length.
    #[must_use]
    pub fn max_response_body(mut self, limit: usize) -> Self {
        self.response_limit = limit;
        self
    }

    /// Validates configuration and creates a client without sending network requests.
    ///
    /// # Errors
    /// Returns an error for invalid credentials, region, timeout, body limits, or a missing transport.
    pub fn build(self) -> Result<Client, SdkError> {
        if self.timeout.is_zero() || self.request_limit == 0 || self.response_limit == 0 {
            return Err(SdkError::Configuration(
                "timeout and body limits must be greater than zero",
            ));
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
            None => {
                return Err(SdkError::Configuration(
                    "a credentials provider must be configured explicitly",
                ));
            }
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
        "provide a Transport or enable transport-reqwest",
    ))
}
