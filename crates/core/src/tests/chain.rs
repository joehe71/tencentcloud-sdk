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
    let missing =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/nonexistent-identity-token");
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
