use std::net::IpAddr;

use http::Uri;

use crate::SdkError;

/// A validated API root endpoint without user information, query strings, or arbitrary paths.
#[derive(Clone, Debug)]
pub struct Endpoint(Uri);

impl Endpoint {
    /// Sets an explicit HTTPS root endpoint, such as <https://cvm.ap-shanghai.tencentcloudapi.com/>.
    ///
    /// # Errors
    /// Returns an error if the URL is not an HTTPS root endpoint.
    pub fn https(url: &str) -> Result<Self, SdkError> {
        let endpoint = Self::parse(url)?;
        if endpoint.0.scheme_str() != Some("https") {
            return Err(SdkError::Configuration("API endpoint must use HTTPS"));
        }
        Ok(endpoint)
    }

    /// Allows plaintext HTTP only for loopback addresses used by local mock servers.
    ///
    /// # Errors
    /// Returns an error if the URL is not an HTTP loopback root endpoint.
    pub fn loopback_http(url: &str) -> Result<Self, SdkError> {
        let endpoint = Self::parse(url)?;
        let host = endpoint.0.host().unwrap_or_default();
        let ip = host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<IpAddr>();
        if endpoint.0.scheme_str() != Some("http") || !ip.is_ok_and(|ip| ip.is_loopback()) {
            return Err(SdkError::Configuration(
                "plaintext HTTP requires a numeric loopback address",
            ));
        }
        Ok(endpoint)
    }

    fn parse(url: &str) -> Result<Self, SdkError> {
        if url.contains('#') {
            return Err(SdkError::Configuration(
                "API endpoint must not contain a fragment",
            ));
        }
        let uri: Uri = url
            .parse()
            .map_err(|_| SdkError::Configuration("invalid API endpoint URL"))?;
        let authority = uri
            .authority()
            .ok_or(SdkError::Configuration("API endpoint has no host"))?;
        if authority.as_str().contains('@')
            || uri.host().is_none_or(str::is_empty)
            || !matches!(uri.path(), "" | "/")
            || uri.query().is_some()
        {
            return Err(SdkError::Configuration(
                "API endpoint must be a root URL without user information or a query string",
            ));
        }
        let uri = Uri::builder()
            .scheme(uri.scheme_str().unwrap_or_default())
            .authority(authority.as_str().to_ascii_lowercase())
            .path_and_query("/")
            .build()
            .map_err(|_| SdkError::Configuration("invalid API endpoint URL"))?;
        Ok(Self(uri))
    }

    pub(crate) fn for_service(service: &str) -> Result<Self, SdkError> {
        Self::https(&format!("https://{service}.tencentcloudapi.com/"))
    }

    /// Returns the validated URI.
    #[must_use]
    pub fn uri(&self) -> &Uri {
        &self.0
    }
}

#[cfg(test)]
#[path = "tests/endpoint.rs"]
mod tests;
