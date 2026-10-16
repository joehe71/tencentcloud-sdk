use std::{fmt, sync::Arc, time::Duration};

use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use serde::{Deserialize, Serialize};

use crate::{
    Credentials, Endpoint, SdkError,
    operation::validate_region,
    protocol,
    runtime::{Clock, SystemClock},
    signing,
    transport::{Transport, TransportRequest},
};

const BODY_LIMIT: usize = 1024 * 1024;

/// Shared credential HTTP resources; construction never reads files or sends requests.
///
/// Reuse the application's transport to share its connection pool. Redirects, retries,
/// and environment proxies must be disabled, as with the built-in reqwest transport.
#[derive(Clone)]
pub struct RoleProviderOptions {
    pub(super) transport: Arc<dyn Transport>,
    pub(super) clock: Arc<dyn Clock>,
    pub(super) timeout: Duration,
    pub(super) refresh_before: Duration,
    sts_endpoint: Option<Endpoint>,
    pub(super) metadata_endpoint: Option<Endpoint>,
}

impl RoleProviderOptions {
    /// Uses an existing transport with a 10-second budget and a 10-minute refresh window.
    #[must_use]
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            clock: Arc::new(SystemClock),
            timeout: Duration::from_secs(10),
            refresh_before: Duration::from_secs(600),
            sts_endpoint: None,
            metadata_endpoint: None,
        }
    }
    /// Overrides the trusted STS root endpoint, for example for private networking or mocks.
    #[must_use]
    pub fn sts_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.sts_endpoint = Some(endpoint);
        self
    }
    /// Overrides the metadata server root; numeric loopback HTTP is available for local tests.
    #[must_use]
    pub fn metadata_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.metadata_endpoint = Some(endpoint);
        self
    }
    /// Sets a fixed or custom wall clock for expiration and signing.
    #[must_use]
    pub fn clock(mut self, clock: impl Clock + 'static) -> Self {
        self.clock = Arc::new(clock);
        self
    }
    /// Bounds each provider call, including waiting for refresh and reading token files.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Sets the refresh window, capped at half the remaining lifetime of a new credential.
    #[must_use]
    pub fn refresh_before(mut self, window: Duration) -> Self {
        self.refresh_before = window;
        self
    }
}

impl fmt::Debug for RoleProviderOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoleProviderOptions")
            .field("timeout", &self.timeout)
            .field("refresh_before", &self.refresh_before)
            .finish_non_exhaustive()
    }
}

pub(super) struct CredentialHttp {
    pub options: RoleProviderOptions,
    sts_endpoint: Endpoint,
}

impl CredentialHttp {
    pub fn new(options: RoleProviderOptions) -> Result<Self, SdkError> {
        if options.timeout.is_zero()
            || options.timeout > Duration::from_secs(300)
            || options.refresh_before.as_secs() > 86_400
        {
            return Err(SdkError::Configuration(
                "credential timeout must be in (0, 300s] and refresh window at most one day",
            ));
        }
        let sts_endpoint = options
            .sts_endpoint
            .clone()
            .map_or_else(|| Endpoint::for_service("sts"), Ok)?;
        Ok(Self {
            options,
            sts_endpoint,
        })
    }

    pub async fn sts<I: Serialize + Sync>(
        &self,
        action: &str,
        region: &str,
        input: &I,
        signer: Option<&Credentials>,
    ) -> Result<ExpiringCredentials, SdkError> {
        validate_region(region)?;
        let body = protocol::encode(input, BODY_LIMIT)?;
        let host = self
            .sts_endpoint
            .uri()
            .authority()
            .ok_or(SdkError::Configuration("STS endpoint has no host"))?
            .as_str();
        let timestamp = self.options.clock.unix_timestamp()?;
        let mut headers = HeaderMap::new();
        for (name, value) in [
            ("host", host),
            ("content-type", signing::CONTENT_TYPE),
            ("x-tc-action", action),
            ("x-tc-version", "2018-08-13"),
            ("x-tc-region", region),
        ] {
            headers.insert(
                http::HeaderName::from_bytes(name.as_bytes())
                    .map_err(|_| SdkError::Configuration("invalid STS header name"))?,
                header(value)?,
            );
        }
        headers.insert("x-tc-timestamp", header(&timestamp.to_string())?);
        let authorization = if let Some(signer) = signer {
            if let Some(token) = signer.token() {
                headers.insert("x-tc-token", token.clone());
            }
            signing::authorization(signer, "sts", host, action, &body, timestamp)?
        } else {
            // Only AssumeRoleWithWebIdentity is permitted to use unsigned STS authentication.
            if action != "AssumeRoleWithWebIdentity" {
                return Err(SdkError::Configuration(
                    "this STS action requires signing credentials",
                ));
            }
            HeaderValue::from_static("SKIP")
        };
        headers.insert("authorization", authorization);
        let response = self
            .options
            .transport
            .send(TransportRequest {
                method: Method::POST,
                uri: self.sts_endpoint.uri().clone(),
                headers,
                body,
            })
            .await?;
        let raw = protocol::collect(response, BODY_LIMIT).await?;
        let response = protocol::decode::<RoleResponse>(&raw)?;
        response.data.into_expiring()
    }

    pub async fn metadata(&self, role_name: Option<&str>) -> Result<ExpiringCredentials, SdkError> {
        let root = self.options.metadata_endpoint.as_ref().map_or_else(
            || "http://metadata.tencentyun.com/".to_owned(),
            |endpoint| endpoint.uri().to_string(),
        );
        let base = format!("{root}latest/meta-data/cam/security-credentials/");
        let role = if let Some(role) = role_name {
            role.to_owned()
        } else {
            let raw = self.get_metadata(&base).await?;
            std::str::from_utf8(&raw)
                .map_err(|_| SdkError::Credentials("metadata role name is not UTF-8"))?
                .trim()
                .to_owned()
        };
        validate_role_name(&role)?;
        let raw = self.get_metadata(&format!("{base}{role}")).await?;
        let response: MetadataResponse = serde_json::from_slice(&raw)
            .map_err(|_| SdkError::Credentials("invalid metadata credentials response"))?;
        if response.code != "Success" {
            return Err(SdkError::Credentials(
                "metadata credentials request was rejected",
            ));
        }
        Ok(ExpiringCredentials {
            credentials: Arc::new(response.credentials.into_credentials()?),
            expires_at: response.expired_time,
        })
    }

    async fn get_metadata(&self, uri: &str) -> Result<Bytes, SdkError> {
        let uri: Uri = uri
            .parse()
            .map_err(|_| SdkError::Configuration("invalid metadata URI"))?;
        let response = self
            .options
            .transport
            .send(TransportRequest {
                method: Method::GET,
                uri,
                headers: HeaderMap::new(),
                body: Bytes::new(),
            })
            .await?;
        let raw = protocol::collect(response, BODY_LIMIT).await?;
        if raw.status != StatusCode::OK {
            return Err(SdkError::Protocol {
                message: "metadata service returned a non-200 response",
                status: raw.status,
                request_id: None,
            });
        }
        Ok(raw.body)
    }
}

fn header(value: &str) -> Result<HeaderValue, SdkError> {
    HeaderValue::from_str(value).map_err(|_| SdkError::Configuration("invalid STS header value"))
}

pub(super) fn validate_role_name(role: &str) -> Result<(), SdkError> {
    if role.is_empty()
        || role.len() > 128
        || matches!(role, "." | "..")
        || !role
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_+=,.@-".contains(&byte))
    {
        return Err(SdkError::Configuration("invalid metadata role name"));
    }
    Ok(())
}

pub(super) struct ExpiringCredentials {
    pub credentials: Arc<Credentials>,
    pub expires_at: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WireCredentials {
    tmp_secret_id: String,
    tmp_secret_key: String,
    token: String,
}
impl WireCredentials {
    fn into_credentials(self) -> Result<Credentials, SdkError> {
        Credentials::new(self.tmp_secret_id, self.tmp_secret_key)?.with_token(self.token)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RoleResponse {
    credentials: WireCredentials,
    expired_time: i64,
}
impl RoleResponse {
    fn into_expiring(self) -> Result<ExpiringCredentials, SdkError> {
        Ok(ExpiringCredentials {
            credentials: Arc::new(self.credentials.into_credentials()?),
            expires_at: self.expired_time,
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MetadataResponse {
    code: String,
    expired_time: i64,
    #[serde(flatten)]
    credentials: WireCredentials,
}
