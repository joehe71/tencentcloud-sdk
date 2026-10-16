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
#[path = "../tests/chain.rs"]
mod tests;
