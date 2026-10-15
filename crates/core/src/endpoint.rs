use std::net::IpAddr;

use http::Uri;

use crate::SdkError;

/// 验证过的 API 根端点，不能包含用户信息、查询串或任意路径。
#[derive(Clone, Debug)]
pub struct Endpoint(Uri);

impl Endpoint {
    /// 显式指定 HTTPS 根端点，例如 <https://cvm.ap-shanghai.tencentcloudapi.com/>。
    ///
    /// # Errors
    /// URL 不是 HTTPS 根端点时返回错误。
    pub fn https(url: &str) -> Result<Self, SdkError> {
        let endpoint = Self::parse(url)?;
        if endpoint.0.scheme_str() != Some("https") {
            return Err(SdkError::Configuration("API 端点必须使用 HTTPS"));
        }
        Ok(endpoint)
    }

    /// 仅允许明文回环地址，供本地模拟服务测试使用，不允许任意明文公网端点。
    ///
    /// # Errors
    /// URL 不是 HTTP 回环根端点时返回错误。
    pub fn loopback_http(url: &str) -> Result<Self, SdkError> {
        let endpoint = Self::parse(url)?;
        let host = endpoint.0.host().unwrap_or_default();
        let ip = host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<IpAddr>();
        if endpoint.0.scheme_str() != Some("http") || !ip.is_ok_and(|ip| ip.is_loopback()) {
            return Err(SdkError::Configuration("明文 HTTP 仅允许数值回环地址"));
        }
        Ok(endpoint)
    }

    fn parse(url: &str) -> Result<Self, SdkError> {
        if url.contains('#') {
            return Err(SdkError::Configuration("API 端点不能包含片段"));
        }
        let uri: Uri = url
            .parse()
            .map_err(|_| SdkError::Configuration("API 端点 URL 无效"))?;
        let authority = uri
            .authority()
            .ok_or(SdkError::Configuration("API 端点缺少主机"))?;
        if authority.as_str().contains('@')
            || uri.host().is_none_or(str::is_empty)
            || !matches!(uri.path(), "" | "/")
            || uri.query().is_some()
        {
            return Err(SdkError::Configuration(
                "API 端点必须为不含用户信息和查询串的根 URL",
            ));
        }
        let uri = Uri::builder()
            .scheme(uri.scheme_str().unwrap_or_default())
            .authority(authority.as_str().to_ascii_lowercase())
            .path_and_query("/")
            .build()
            .map_err(|_| SdkError::Configuration("API 端点 URL 无效"))?;
        Ok(Self(uri))
    }

    pub(crate) fn for_service(service: &str) -> Result<Self, SdkError> {
        Self::https(&format!("https://{service}.tencentcloudapi.com/"))
    }

    /// 取得已验证的 URI。
    #[must_use]
    pub fn uri(&self) -> &Uri {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_roots_and_keeps_ports() {
        assert_eq!(
            Endpoint::https("https://CVM.tencentcloudapi.com:8443")
                .unwrap()
                .uri(),
            "https://cvm.tencentcloudapi.com:8443/"
        );
        for url in [
            "http://example.com",
            "https://id:key@example.com",
            "https://example.com/api",
            "https://example.com/?a=1",
            "https://example.com/#secret",
            "not-a-url",
        ] {
            assert!(Endpoint::https(url).is_err(), "{url}");
        }
        assert!(Endpoint::loopback_http("http://127.0.0.1:8080/").is_ok());
        assert!(Endpoint::loopback_http("http://[::1]:8080/").is_ok());
        assert!(Endpoint::loopback_http("http://example.com/").is_err());
        assert!(Endpoint::loopback_http("http://localhost/").is_err());
    }
}
