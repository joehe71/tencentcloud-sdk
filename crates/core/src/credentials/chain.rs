use std::{collections::BTreeMap, fmt, path::PathBuf, sync::Arc};

use tokio::{sync::OnceCell, time::timeout};

use super::{
    Credentials, CvmRoleCredentialsProvider, OidcRoleConfig, OidcRoleCredentialsProvider,
    ProfileCredentialsProvider, ProvideCredentials, RoleProviderOptions, StaticCredentialsProvider,
    profile::default_paths, role_http::CredentialHttp, roles::environment,
};
use crate::{BoxFuture, SdkError};

/// An explicitly selected, lazy default credentials chain.
///
/// Tries environment, profile files, configured TKE OIDC, then CVM metadata. A configured
/// but invalid source fails closed. The selected identity is pinned for this provider's
/// lifetime; TKE does not silently fall back to a node's identity after an error.
#[derive(Clone)]
pub struct DefaultCredentialsProvider(Arc<Inner>);
struct Inner {
    options: RoleProviderOptions,
    selected: OnceCell<Arc<dyn ProvideCredentials>>,
}

impl DefaultCredentialsProvider {
    /// Creates a chain without reading files, environment variables, or metadata.
    ///
    /// # Errors
    /// Returns an error for invalid HTTP or refresh options.
    pub fn new(options: RoleProviderOptions) -> Result<Self, SdkError> {
        CredentialHttp::new(options.clone())?;
        Ok(Self(Arc::new(Inner {
            options,
            selected: OnceCell::new(),
        })))
    }
}
impl ProvideCredentials for DefaultCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        Box::pin(async {
            timeout(self.0.options.timeout, async {
                let provider = self
                    .0
                    .selected
                    .get_or_try_init(|| {
                        select(environment, default_paths(), self.0.options.clone())
                    })
                    .await?;
                provider.provide_credentials().await
            })
            .await
            .map_err(|_| SdkError::Timeout)?
        })
    }
}
impl fmt::Debug for DefaultCredentialsProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DefaultCredentialsProvider")
            .finish_non_exhaustive()
    }
}

async fn select(
    mut lookup: impl FnMut(&str) -> Result<Option<String>, SdkError> + Send,
    paths: Vec<PathBuf>,
    options: RoleProviderOptions,
) -> Result<Arc<dyn ProvideCredentials>, SdkError> {
    let id = lookup("TENCENTCLOUD_SECRET_ID")?;
    let key = lookup("TENCENTCLOUD_SECRET_KEY")?;
    let token = lookup("TENCENTCLOUD_TOKEN")?;
    if id.is_some() || key.is_some() || token.is_some() {
        let mut credentials = Credentials::new(
            id.ok_or(SdkError::Credentials("missing TENCENTCLOUD_SECRET_ID"))?,
            key.ok_or(SdkError::Credentials("missing TENCENTCLOUD_SECRET_KEY"))?,
        )?;
        if let Some(token) = token {
            credentials = credentials.with_token(token)?;
        }
        return Ok(Arc::new(StaticCredentialsProvider::new(credentials)));
    }
    for path in paths {
        if let Some(provider) = ProfileCredentialsProvider::try_load(&path).await? {
            return Ok(Arc::new(provider));
        }
    }
    let mut tke = BTreeMap::new();
    for name in [
        "TKE_REGION",
        "TKE_PROVIDER_ID",
        "TKE_ROLE_ARN",
        "TKE_WEB_IDENTITY_TOKEN_FILE",
    ] {
        if let Some(value) = lookup(name)? {
            tke.insert(name, value);
        }
    }
    if !tke.is_empty() {
        let config = OidcRoleConfig::from_lookup(|name| Ok(tke.remove(name)))?;
        return Ok(Arc::new(OidcRoleCredentialsProvider::new(config, options)?));
    }
    Ok(Arc::new(CvmRoleCredentialsProvider::new(None, options)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::{Transport, TransportRequest, TransportResponse};
    use bytes::Bytes;
    use http::{HeaderMap, StatusCode};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MetadataMock(Arc<AtomicUsize>);
    impl Transport for MetadataMock {
        fn send(
            &self,
            request: TransportRequest,
        ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                assert_eq!(request.method, http::Method::GET);
                let body = if request.uri.path().ends_with("security-credentials/") {
                    Bytes::from_static(b"test-role")
                } else {
                    Bytes::from_static(br#"{"Code":"Success","TmpSecretId":"test-id","TmpSecretKey":"test-key","Token":"test-token","ExpiredTime":4102444800}"#)
                };
                Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    body,
                ))
            })
        }
    }
    fn options() -> (RoleProviderOptions, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        (
            RoleProviderOptions::new(Arc::new(MetadataMock(Arc::clone(&calls)))),
            calls,
        )
    }
    fn lookup<'a>(
        values: &'a [(&str, &str)],
    ) -> impl FnMut(&str) -> Result<Option<String>, SdkError> + Send + 'a {
        move |name| {
            Ok(values
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned()))
        }
    }

    #[tokio::test]
    async fn configured_environment_wins_without_reading_lower_priority_sources() {
        let (options, calls) = options();
        let values = [
            ("TENCENTCLOUD_SECRET_ID", "test-id"),
            ("TENCENTCLOUD_SECRET_KEY", "test-key"),
            ("TENCENTCLOUD_TOKEN", "test-token"),
            ("TKE_REGION", "partial-configuration"),
        ];
        let invalid_profile = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let provider = select(lookup(&values), vec![invalid_profile], options)
            .await
            .unwrap();
        let credentials = provider.provide_credentials().await.unwrap();
        assert_eq!(credentials.token().unwrap(), "test-token");
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn partial_environment_or_invalid_profile_does_not_fall_through() {
        let (opts, calls) = options();
        assert!(
            select(
                lookup(&[("TENCENTCLOUD_SECRET_ID", "test-id")]),
                Vec::new(),
                opts.clone()
            )
            .await
            .is_err()
        );
        let invalid_profile = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(
            select(lookup(&[]), vec![invalid_profile], opts)
                .await
                .is_err()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn configured_tke_identity_never_falls_back_to_node_metadata() {
        let (opts, calls) = options();
        let missing = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/nonexistent-identity-token");
        let values = [
            ("TKE_REGION", "ap-shanghai"),
            ("TKE_PROVIDER_ID", "test-provider"),
            ("TKE_ROLE_ARN", "qcs::cam::uin/123:roleName/test"),
            ("TKE_WEB_IDENTITY_TOKEN_FILE", missing.to_str().unwrap()),
        ];
        let provider = select(lookup(&values), Vec::new(), opts.clone())
            .await
            .unwrap();
        assert!(provider.provide_credentials().await.is_err());
        assert!(provider.provide_credentials().await.is_err());
        assert!(
            select(lookup(&[("TKE_REGION", "ap-shanghai")]), Vec::new(), opts)
                .await
                .is_err()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn absent_sources_select_metadata_without_probing_during_selection() {
        let (opts, calls) = options();
        let provider = select(lookup(&[]), Vec::new(), opts).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        provider.provide_credentials().await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }
}
