use crate::{
    Error,
    protocol::{self, configuration},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use md5::{Digest, Md5};
use serde::de::DeserializeOwned;
use std::{fmt, sync::Arc, time::Duration};
use tencentcloud_sdk_core::{
    Credentials, Endpoint, SdkError, cos_signing,
    credentials::{ProvideCredentials, StaticCredentialsProvider},
    runtime::{Clock, SystemClock},
    transport::{Transport, TransportRequest},
};
use tokio::time::{Instant, timeout_at};
mod operations;

/// A response retaining COS headers and `RequestId`; default formatting omits business data.
pub struct Response<T> {
    /// Decoded payload or buffered object bytes.
    pub data: T,
    /// HTTP status.
    pub status: StatusCode,
    /// COS request identifier.
    pub request_id: Option<String>,
    /// Response headers, including `ETag`, Content-Length, metadata and version identifiers.
    pub headers: HeaderMap,
}
impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CosResponse")
            .field("status", &self.status)
            .field("request_id", &self.request_id)
            .finish_non_exhaustive()
    }
}
/// A bearer URL; explicitly access its string only when sharing it with an authorized consumer.
pub struct PresignedUrl(String);
impl PresignedUrl {
    /// Returns the sensitive URL. Do not log it or persist it in diagnostics.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Debug for PresignedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PresignedUrl { .. }")
    }
}

/// An inexpensive clone sharing one immutable configuration and credential provider.
#[derive(Clone)]
pub struct Client(Arc<Inner>);
struct Inner {
    endpoint: Endpoint,
    service_endpoint: Endpoint,
    credentials: Arc<dyn ProvideCredentials>,
    transport: Arc<dyn Transport>,
    clock: Arc<dyn Clock>,
    timeout: Duration,
    request_limit: usize,
    response_limit: usize,
    signature_validity: Duration,
}
/// Explicit COS configuration; no files, environment, or cloud service are accessed at construction.
pub struct ClientBuilder {
    bucket: String,
    region: String,
    endpoint: Option<Endpoint>,
    service_endpoint: Option<Endpoint>,
    credentials: Option<Arc<dyn ProvideCredentials>>,
    transport: Option<Arc<dyn Transport>>,
    clock: Arc<dyn Clock>,
    timeout: Duration,
    request_limit: usize,
    response_limit: usize,
}
impl Client {
    /// Configures a full bucket name including APPID and its actual COS region.
    #[must_use]
    pub fn builder(bucket: impl Into<String>, region: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            bucket: bucket.into(),
            region: region.into(),
            endpoint: None,
            service_endpoint: None,
            credentials: None,
            transport: None,
            clock: Arc::new(SystemClock),
            timeout: Duration::from_secs(60),
            request_limit: 32 * 1024 * 1024,
            response_limit: 32 * 1024 * 1024,
        }
    }
    /// Creates a GET, HEAD, PUT or DELETE URL without sending a request.
    ///
    /// Only Host is required for the consumer. Lifetime must be in [1s, 7d], but temporary
    /// credentials can expire earlier. Use short lifetimes and explicitly authorize URL sharing.
    ///
    /// # Errors
    /// Returns validation, credential, signing or deadline errors.
    pub async fn presign(
        &self,
        method: Method,
        key: &str,
        validity: Duration,
    ) -> Result<PresignedUrl, Error> {
        if !matches!(
            method,
            Method::GET | Method::HEAD | Method::PUT | Method::DELETE
        ) {
            return Err(configuration("unsupported presigned COS method"));
        }
        let deadline = self.deadline()?;
        timeout_at(deadline, async {
            let request = self.prepare(Request::new(method, Some(key)))?;
            let credentials = self.0.credentials.provide_credentials().await?;
            let value = cos_signing::presign(
                &credentials,
                &request,
                self.0.clock.unix_timestamp()?,
                validity,
            )?;
            check_deadline(deadline)?;
            Ok(PresignedUrl(value))
        })
        .await
        .map_err(|_| SdkError::Timeout)?
    }
    fn deadline(&self) -> Result<Instant, Error> {
        Instant::now()
            .checked_add(self.0.timeout)
            .ok_or_else(|| configuration("COS timeout is out of range"))
    }
    fn prepare(&self, request: Request<'_>) -> Result<TransportRequest, Error> {
        if request.body.len() > self.0.request_limit {
            return Err(SdkError::BodyTooLarge {
                limit: self.0.request_limit,
            }
            .into());
        }
        let path = match request.key {
            Some(key) => key_path(key)?,
            None => "/".to_owned(),
        };
        let root = if request.service {
            &self.0.service_endpoint
        } else {
            &self.0.endpoint
        };
        let query = request
            .query
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}={}",
                    cos_signing::encode_component(key),
                    cos_signing::encode_component(value)
                )
            })
            .collect::<Vec<_>>()
            .join("&");
        let uri = format!(
            "{}{path}{}{query}",
            root.uri().to_string().trim_end_matches('/'),
            if query.is_empty() { "" } else { "?" }
        )
        .parse()
        .map_err(|_| configuration("invalid COS request URI"))?;
        Ok(TransportRequest {
            method: request.method,
            uri,
            headers: request.headers,
            body: request.body,
        })
    }
    async fn call(&self, request: Request<'_>) -> Result<Response<Bytes>, Error> {
        let deadline = self.deadline()?;
        timeout_at(deadline, async {
            let mut request = self.prepare(request)?;
            if matches!(request.method, Method::PUT | Method::POST) {
                request
                    .headers
                    .insert("content-length", header(&request.body.len().to_string())?);
                request.headers.insert(
                    "content-md5",
                    header(&STANDARD.encode(Md5::digest(&request.body)))?,
                );
            }
            check_deadline(deadline)?;
            let credentials = self.0.credentials.provide_credentials().await?;
            cos_signing::authorize(
                &credentials,
                &mut request,
                self.0.clock.unix_timestamp()?,
                self.0.signature_validity,
            )?;
            check_deadline(deadline)?;
            let response = self.0.transport.send(request).await?;
            let response = protocol::collect(response, self.0.response_limit).await?;
            check_deadline(deadline)?;
            Ok(response)
        })
        .await
        .map_err(|_| SdkError::Timeout)?
    }
    async fn xml<T: DeserializeOwned>(
        &self,
        request: Request<'_>,
        root: &str,
    ) -> Result<Response<T>, Error> {
        let deadline = self.deadline()?;
        let response = protocol::decode(self.call(request).await?, root);
        check_deadline(deadline)?;
        response
    }
    async fn empty(&self, request: Request<'_>) -> Result<Response<()>, Error> {
        protocol::ensure_success_empty(self.call(request).await?)
    }
}
impl ClientBuilder {
    /// Supplies a static immutable credential snapshot.
    #[must_use]
    pub fn credentials(self, credentials: Credentials) -> Self {
        self.credentials_provider(StaticCredentialsProvider::new(credentials))
    }
    /// Shares a custom provider, including OIDC, CVM or `AssumeRole` providers.
    #[must_use]
    pub fn credentials_provider(mut self, provider: impl ProvideCredentials + 'static) -> Self {
        self.credentials = Some(Arc::new(provider));
        self
    }
    /// Shares the same transport and pool used for credential HTTP and other SDK clients.
    #[must_use]
    pub fn shared_transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }
    /// Overrides the bucket root with a trusted HTTPS or explicit loopback test endpoint.
    #[must_use]
    pub fn endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    /// Overrides the GET Service root independently of the bucket endpoint.
    #[must_use]
    pub fn service_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.service_endpoint = Some(endpoint);
        self
    }
    /// Supplies a signing clock, commonly fixed for offline tests.
    #[must_use]
    pub fn clock(mut self, clock: impl Clock + 'static) -> Self {
        self.clock = Arc::new(clock);
        self
    }
    /// Sets the complete per-call budget, including credential acquisition and body reads.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Sets the maximum buffered upload or XML request size.
    #[must_use]
    pub fn max_request_body(mut self, limit: usize) -> Self {
        self.request_limit = limit;
        self
    }
    /// Sets the maximum buffered download or XML response size.
    #[must_use]
    pub fn max_response_body(mut self, limit: usize) -> Self {
        self.response_limit = limit;
        self
    }
    /// Validates configuration without reading credentials or sending requests.
    ///
    /// # Errors
    /// Returns an error for invalid names, zero budgets/limits, absent credentials or transport.
    pub fn build(self) -> Result<Client, Error> {
        validate_bucket(&self.bucket)?;
        if self.region.is_empty()
            || !self
                .region
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(configuration("invalid COS region"));
        }
        if self.timeout.is_zero() || self.request_limit == 0 || self.response_limit == 0 {
            return Err(configuration(
                "COS timeout and body limits must be positive",
            ));
        }
        let endpoint = self.endpoint.map_or_else(
            || {
                Endpoint::https(&format!(
                    "https://{}.cos.{}.myqcloud.com/",
                    self.bucket, self.region
                ))
            },
            Ok,
        )?;
        let service_endpoint = self
            .service_endpoint
            .map_or_else(|| Endpoint::https("https://service.cos.myqcloud.com/"), Ok)?;
        let credentials = self
            .credentials
            .ok_or_else(|| configuration("COS credentials must be explicitly configured"))?;
        let transport = match self.transport {
            Some(transport) => transport,
            None => default_transport()?,
        };
        Ok(Client(Arc::new(Inner {
            endpoint,
            service_endpoint,
            credentials,
            transport,
            clock: self.clock,
            timeout: self.timeout,
            request_limit: self.request_limit,
            response_limit: self.response_limit,
            signature_validity: Duration::from_secs(900),
        })))
    }
}
fn default_transport() -> Result<Arc<dyn Transport>, Error> {
    #[cfg(feature = "transport-reqwest")]
    {
        Ok(Arc::new(
            tencentcloud_sdk_core::transport::ReqwestTransport::new(Duration::from_secs(10))?,
        ))
    }
    #[cfg(not(feature = "transport-reqwest"))]
    {
        Err(configuration("COS transport must be explicitly configured"))
    }
}
fn header(value: &str) -> Result<HeaderValue, Error> {
    HeaderValue::from_str(value).map_err(|_| configuration("invalid COS header value"))
}
fn check_deadline(deadline: Instant) -> Result<(), Error> {
    if Instant::now() >= deadline {
        Err(SdkError::Timeout.into())
    } else {
        Ok(())
    }
}
fn validate_bucket(bucket: &str) -> Result<(), Error> {
    let valid_suffix = bucket.rsplit_once('-').is_some_and(|(name, appid)| {
        !name.is_empty() && !appid.is_empty() && appid.bytes().all(|byte| byte.is_ascii_digit())
    });
    if !(3..=63).contains(&bucket.len())
        || !valid_suffix
        || bucket.starts_with('-')
        || !bucket
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(configuration(
            "invalid COS bucket name; include the numeric APPID suffix",
        ));
    }
    Ok(())
}
fn key_path(key: &str) -> Result<String, Error> {
    if key.is_empty()
        || key.len() > 850
        || key.chars().any(char::is_control)
        || key.split('/').any(|part| matches!(part, "." | ".."))
    {
        return Err(configuration(
            "invalid COS key or unsupported dot path segment",
        ));
    }
    Ok(format!(
        "/{}",
        key.split('/')
            .map(cos_signing::encode_component)
            .collect::<Vec<_>>()
            .join("/")
    ))
}
struct Request<'a> {
    method: Method,
    key: Option<&'a str>,
    query: Vec<(&'static str, String)>,
    headers: HeaderMap,
    body: Bytes,
    service: bool,
}
impl<'a> Request<'a> {
    fn new(method: Method, key: Option<&'a str>) -> Self {
        Self {
            method,
            key,
            query: Vec::new(),
            headers: HeaderMap::new(),
            body: Bytes::new(),
            service: false,
        }
    }
}
