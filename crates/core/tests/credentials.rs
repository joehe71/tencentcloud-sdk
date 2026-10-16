//! Offline role-credential tests; no ambient credentials or cloud endpoints are consulted.
#![cfg(feature = "credentials-roles")]

use std::{
    future::{Future, pending},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicI64, AtomicUsize, Ordering},
    },
    time::Duration,
};

use bytes::Bytes;
use http::{HeaderMap, Method, StatusCode};
use serde_json::{Value, json};
use tencentcloud_sdk_foundation::{
    ApiResponse, BoxFuture, Client, Credentials, Endpoint, Operation, SdkError,
    credentials::{
        AssumeRoleConfig, AssumeRoleCredentialsProvider, CvmRoleCredentialsProvider,
        DefaultCredentialsProvider, OidcRoleConfig, OidcRoleCredentialsProvider,
        ProfileCredentialsProvider, ProvideCredentials, RoleProviderOptions,
        StaticCredentialsProvider, WebIdentityToken,
    },
    runtime::Clock,
    transport::{Transport, TransportRequest, TransportResponse},
};

const NOW: i64 = 1_700_000_000;
const ARN: &str = "qcs::cam::uin/123456789:roleName/test-role";
type Handler = dyn Fn(TransportRequest) -> BoxFuture<'static, Result<TransportResponse, SdkError>>
    + Send
    + Sync;
#[derive(Clone)]
struct Mock {
    handler: Arc<Handler>,
    calls: Arc<AtomicUsize>,
}
impl Mock {
    fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(TransportRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<TransportResponse, SdkError>> + Send + 'static,
    {
        Self {
            handler: Arc::new(move |request| Box::pin(handler(request))),
            calls: Arc::new(AtomicUsize::new(0)),
        }
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl Transport for Mock {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        (self.handler)(request)
    }
}
#[derive(Clone)]
struct TestClock(Arc<AtomicI64>);
impl TestClock {
    fn new() -> Self {
        Self(Arc::new(AtomicI64::new(NOW)))
    }
    fn set(&self, now: i64) {
        self.0.store(now, Ordering::SeqCst);
    }
}
impl Clock for TestClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
fn options(mock: &Mock, clock: &TestClock) -> RoleProviderOptions {
    RoleProviderOptions::new(Arc::new(mock.clone()))
        .clock(clock.clone())
        .sts_endpoint(Endpoint::loopback_http("http://127.0.0.1:12345/").unwrap())
        .metadata_endpoint(Endpoint::loopback_http("http://127.0.0.1:12346/").unwrap())
}
fn response(body: impl serde::Serialize) -> TransportResponse {
    TransportResponse::from_bytes(
        StatusCode::OK,
        HeaderMap::new(),
        serde_json::to_vec(&body).unwrap(),
    )
}
fn issued(expires: i64) -> TransportResponse {
    response(
        json!({"Response":{"Credentials":{"TmpSecretId":"issued-id","TmpSecretKey":"issued-key","Token":"issued-token"},"ExpiredTime":expires,"RequestId":"sts-request"}}),
    )
}
fn oidc(options: RoleProviderOptions, token: WebIdentityToken) -> OidcRoleCredentialsProvider {
    OidcRoleCredentialsProvider::new(
        OidcRoleConfig::new("ap-shanghai", "test-provider", ARN, token)
            .session_name("test-session"),
        options,
    )
    .unwrap()
}
fn inline() -> WebIdentityToken {
    WebIdentityToken::Inline("fake-identity-token".to_owned())
}

struct TestFile {
    directory: PathBuf,
    path: PathBuf,
}
impl TestFile {
    fn new(text: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/credential-tests");
        std::fs::create_dir_all(&base).unwrap();
        let directory = loop {
            let directory = base.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match std::fs::create_dir(&directory) {
                Ok(()) => break directory,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("cannot create test directory: {error}"),
            }
        };
        let path = directory.join("token");
        std::fs::write(&path, text).unwrap();
        Self { directory, path }
    }
}
impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

#[tokio::test]
async fn oidc_uses_skip_auth_and_business_call_uses_the_issued_triple() {
    let mock = Mock::new(|request| async move {
        if request.headers["x-tc-action"] == "AssumeRoleWithWebIdentity" {
            assert_eq!(request.method, Method::POST);
            assert_eq!(request.uri, "http://127.0.0.1:12345/");
            assert_eq!(request.headers["authorization"], "SKIP");
            assert!(!request.headers.contains_key("x-tc-token"));
            assert_eq!(request.headers["x-tc-version"], "2018-08-13");
            assert_eq!(request.headers["x-tc-region"], "ap-shanghai");
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(body["ProviderId"], "test-provider");
            assert_eq!(body["WebIdentityToken"], "fake-identity-token");
            assert_eq!(body["RoleArn"], ARN);
            assert_eq!(body["RoleSessionName"], "test-session");
            assert_eq!(body["DurationSeconds"], 7200);
            assert!(!format!("{request:?}").contains("fake-identity-token"));
            Ok(issued(NOW + 3600))
        } else {
            assert!(
                request.headers["authorization"]
                    .to_str()
                    .unwrap()
                    .contains("Credential=issued-id/")
            );
            assert_eq!(request.headers["x-tc-token"], "issued-token");
            assert!(request.headers["x-tc-token"].is_sensitive());
            Ok(response(json!({"Response":{"RequestId":"business-id"}})))
        }
    });
    let clock = TestClock::new();
    let provider = oidc(options(&mock, &clock), inline());
    assert_eq!(mock.calls(), 0);
    let client = Client::builder()
        .credentials_provider(provider)
        .transport(mock.clone())
        .clock(clock)
        .build()
        .unwrap();
    let operation = Operation::new("cvm", "2017-03-12", "DescribeInstances").unwrap();
    for _ in 0..2 {
        let reply: ApiResponse<Value> = client.execute(&operation, &json!({})).await.unwrap();
        assert_eq!(reply.metadata.request_id.as_deref(), Some("business-id"));
    }
    assert_eq!(mock.calls(), 3);
}

#[tokio::test]
async fn cache_shares_snapshots_refreshes_once_and_rejects_expired_replacements() {
    let clock = TestClock::new();
    let observed_clock = clock.clone();
    let mock = Mock::new(move |_| {
        let now = observed_clock.0.load(Ordering::SeqCst);
        async move {
            tokio::task::yield_now().await;
            Ok(issued(now + 3600))
        }
    });
    let provider = oidc(options(&mock, &clock), inline());
    let mut tasks = Vec::new();
    for _ in 0..32 {
        let provider = provider.clone();
        tasks.push(tokio::spawn(async move {
            provider.provide_credentials().await.unwrap()
        }));
    }
    let first = tasks.remove(0).await.unwrap();
    for task in tasks {
        assert!(Arc::ptr_eq(&first, &task.await.unwrap()));
    }
    assert_eq!(mock.calls(), 1);
    clock.set(NOW + 2999);
    assert!(Arc::ptr_eq(
        &first,
        &provider.provide_credentials().await.unwrap()
    ));
    clock.set(NOW + 3000);
    let second = provider.provide_credentials().await.unwrap();
    assert!(!Arc::ptr_eq(&first, &second));
    assert_eq!(mock.calls(), 2);
    let expired = Mock::new(|_| async { Ok(issued(NOW)) });
    let bad = oidc(options(&expired, &TestClock::new()), inline());
    assert!(matches!(
        bad.provide_credentials().await,
        Err(SdkError::Credentials(_))
    ));
}

#[tokio::test]
async fn projected_token_file_is_reopened_after_rotation() {
    let file = TestFile::new("first-token\n");
    let seen = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&seen);
    let mock = Mock::new(move |request| {
        let attempt = counter.fetch_add(1, Ordering::SeqCst);
        async move {
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(
                body["WebIdentityToken"],
                if attempt == 0 {
                    "first-token"
                } else {
                    "second-token"
                }
            );
            Ok(issued(NOW + 3600 + i64::try_from(attempt).unwrap() * 3000))
        }
    });
    let clock = TestClock::new();
    let provider = oidc(
        options(&mock, &clock),
        WebIdentityToken::File(file.path.clone()),
    );
    provider.provide_credentials().await.unwrap();
    let replacement = file.directory.join("replacement");
    std::fs::write(&replacement, "second-token\n").unwrap();
    std::fs::rename(&replacement, &file.path).unwrap();
    clock.set(NOW + 3000);
    provider.provide_credentials().await.unwrap();
    assert_eq!(seen.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn signed_assume_role_retains_the_source_session_token() {
    let mock = Mock::new(|request| async move {
        assert_eq!(request.headers["x-tc-action"], "AssumeRole");
        assert!(
            request.headers["authorization"]
                .to_str()
                .unwrap()
                .starts_with("TC3-HMAC-SHA256 Credential=source-id/")
        );
        assert!(request.headers["authorization"].is_sensitive());
        assert_eq!(request.headers["x-tc-token"], "source-token");
        assert!(request.headers["x-tc-token"].is_sensitive());
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["RoleArn"], ARN);
        assert!(!body.to_string().contains("source-key"));
        Ok(issued(NOW + 3600))
    });
    let source = StaticCredentialsProvider::new(
        Credentials::new("source-id", "source-key")
            .unwrap()
            .with_token("source-token")
            .unwrap(),
    );
    let provider = AssumeRoleCredentialsProvider::new(
        Arc::new(source),
        AssumeRoleConfig::new("ap-shanghai", ARN),
        options(&mock, &TestClock::new()),
    )
    .unwrap();
    provider.provide_credentials().await.unwrap();
    provider.provide_credentials().await.unwrap();
    assert_eq!(mock.calls(), 1);
}

#[tokio::test]
async fn metadata_discovers_and_caches_the_instance_role() {
    let mock = Mock::new(|request| async move {
        assert_eq!(request.method, Method::GET);
        assert!(request.body.is_empty());
        assert!(!request.headers.contains_key("authorization"));
        if request.uri.path().ends_with("security-credentials/") {
            Ok(TransportResponse::from_bytes(
                StatusCode::OK,
                HeaderMap::new(),
                Bytes::from_static(b"test-role\n"),
            ))
        } else {
            assert_eq!(
                request.uri.path(),
                "/latest/meta-data/cam/security-credentials/test-role"
            );
            Ok(response(
                json!({"Code":"Success","TmpSecretId":"metadata-id","TmpSecretKey":"metadata-key","Token":"metadata-token","ExpiredTime":NOW+3600}),
            ))
        }
    });
    let provider =
        CvmRoleCredentialsProvider::new(None, options(&mock, &TestClock::new())).unwrap();
    let first = provider.provide_credentials().await.unwrap();
    assert!(Arc::ptr_eq(
        &first,
        &provider.provide_credentials().await.unwrap()
    ));
    assert_eq!(mock.calls(), 2);
}

#[tokio::test]
async fn metadata_rejects_path_injection_and_error_responses() {
    let mock = Mock::new(|_| async {
        Ok(TransportResponse::from_bytes(
            StatusCode::OK,
            HeaderMap::new(),
            Bytes::from_static(b"../../another-endpoint"),
        ))
    });
    let opts = options(&mock, &TestClock::new());
    assert!(CvmRoleCredentialsProvider::new(Some("../bad".into()), opts.clone()).is_err());
    let provider = CvmRoleCredentialsProvider::new(None, opts).unwrap();
    assert!(provider.provide_credentials().await.is_err());
    assert_eq!(mock.calls(), 1);
    let denied = Mock::new(|_| async {
        Ok(response(
            json!({"Code":"Denied","ExpiredTime":NOW+3600,"TmpSecretId":"id","TmpSecretKey":"key","Token":"token"}),
        ))
    });
    let provider =
        CvmRoleCredentialsProvider::new(Some("role".into()), options(&denied, &TestClock::new()))
            .unwrap();
    assert!(provider.provide_credentials().await.is_err());
}

#[tokio::test]
async fn refresh_failure_preserves_service_error_without_returning_stale_credentials() {
    let clock = TestClock::new();
    let count = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&count);
    let mock = Mock::new(move |_| {
        let attempt = counter.fetch_add(1, Ordering::SeqCst);
        async move {
            Ok(if attempt == 0 {
                issued(NOW + 3600)
            } else {
                response(
                    json!({"Response":{"Error":{"Code":"AuthFailure.InvalidIdentityToken","Message":"private service detail"},"RequestId":"failure-request"}}),
                )
            })
        }
    });
    let provider = oidc(options(&mock, &clock), inline());
    provider.provide_credentials().await.unwrap();
    clock.set(NOW + 3000);
    let error = provider.provide_credentials().await.unwrap_err();
    assert!(!format!("{error:?}").contains("private service detail"));
    match error {
        SdkError::Service(error) => {
            assert_eq!(error.code, "AuthFailure.InvalidIdentityToken");
            assert_eq!(error.request_id.as_deref(), Some("failure-request"));
        }
        error => panic!("unexpected error: {error}"),
    }
    for _ in 0..32 {
        assert!(provider.provide_credentials().await.is_err());
    }
    assert_eq!(mock.calls(), 2);
    clock.set(NOW + 4000);
    assert!(provider.provide_credentials().await.is_err());
    assert_eq!(mock.calls(), 2);
}

#[tokio::test]
async fn cancellation_and_timeout_release_the_refresh_gate() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&attempts);
    let mock = Mock::new(move |_| {
        let attempt = counter.fetch_add(1, Ordering::SeqCst);
        async move {
            if attempt == 0 {
                pending::<()>().await;
            }
            Ok(issued(NOW + 3600))
        }
    });
    let provider = oidc(
        options(&mock, &TestClock::new()).timeout(Duration::from_millis(20)),
        inline(),
    );
    assert!(matches!(
        provider.provide_credentials().await,
        Err(SdkError::Timeout)
    ));
    provider.provide_credentials().await.unwrap();
    assert_eq!(mock.calls(), 2);

    let attempts = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&attempts);
    let started = Arc::new(tokio::sync::Notify::new());
    let signal = Arc::clone(&started);
    let mock = Mock::new(move |_| {
        let attempt = counter.fetch_add(1, Ordering::SeqCst);
        let signal = Arc::clone(&signal);
        async move {
            if attempt == 0 {
                signal.notify_one();
                pending::<()>().await;
            }
            Ok(issued(NOW + 3600))
        }
    });
    let provider = oidc(options(&mock, &TestClock::new()), inline());
    let worker = provider.clone();
    let task = tokio::spawn(async move { worker.provide_credentials().await });
    started.notified().await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    provider.provide_credentials().await.unwrap();
    assert_eq!(mock.calls(), 2);
}

#[tokio::test]
async fn token_files_are_bounded_and_invalid_tokens_do_not_reach_transport() {
    let mock = Mock::new(|_| async { panic!("invalid token must not reach transport") });
    for token in [String::new(), "x".repeat(64 * 1024 + 1)] {
        let file = TestFile::new(&token);
        let provider = oidc(
            options(&mock, &TestClock::new()),
            WebIdentityToken::File(file.path.clone()),
        );
        assert!(provider.provide_credentials().await.is_err());
    }
    let file = TestFile::new("unused");
    std::fs::write(&file.path, [0xff]).unwrap();
    assert!(
        oidc(
            options(&mock, &TestClock::new()),
            WebIdentityToken::File(file.path.clone())
        )
        .provide_credentials()
        .await
        .is_err()
    );
    assert_eq!(mock.calls(), 0);
}

#[tokio::test]
async fn malformed_or_incomplete_sts_credentials_are_not_cached() {
    for body in [
        json!({"Response":{"Credentials":{"TmpSecretId":"id","TmpSecretKey":"key"},"ExpiredTime":NOW+3600,"RequestId":"bad-response"}}),
        json!({"Response":{"Credentials":{"TmpSecretId":"id","TmpSecretKey":"key","Token":""},"ExpiredTime":NOW+3600}}),
        json!({"Response":{"Credentials":{"TmpSecretId":"id","TmpSecretKey":"key","Token":"token"},"ExpiredTime":NOW-1}}),
    ] {
        let mock = Mock::new(move |_| {
            let body = body.clone();
            async move { Ok(response(body)) }
        });
        let provider = oidc(options(&mock, &TestClock::new()), inline());
        assert!(provider.provide_credentials().await.is_err());
        assert!(provider.provide_credentials().await.is_err());
        assert_eq!(mock.calls(), 1);
    }
}

#[test]
fn invalid_options_and_configuration_fail_before_io_and_debug_is_redacted() {
    let mock = Mock::new(|_| async { panic!("construction must not send requests") });
    let opts = options(&mock, &TestClock::new());
    assert!(DefaultCredentialsProvider::new(opts.clone().timeout(Duration::ZERO)).is_err());
    assert!(
        DefaultCredentialsProvider::new(opts.clone().refresh_before(Duration::from_secs(86_401)))
            .is_err()
    );
    let bad = OidcRoleConfig::new("ap-shanghai", "provider", ARN, inline()).duration_seconds(1);
    assert!(OidcRoleCredentialsProvider::new(bad, opts.clone()).is_err());
    let bad =
        OidcRoleConfig::new("ap-shanghai", "provider", ARN, inline()).session_name("bad/name");
    assert!(OidcRoleCredentialsProvider::new(bad, opts.clone()).is_err());
    assert!(!format!("{:?}", inline()).contains("fake-identity-token"));
    assert!(!format!("{:?}", oidc(opts.clone(), inline())).contains("fake-identity-token"));
    DefaultCredentialsProvider::new(opts).unwrap();
    assert_eq!(mock.calls(), 0);
}

#[tokio::test]
async fn profile_files_are_explicit_and_support_temporary_tokens() {
    let file = TestFile::new(
        "[default]\nsecret_id=profile-id\nsecret_key=profile-key\ntoken=profile-token\n",
    );
    let provider = ProfileCredentialsProvider::load(&file.path).await.unwrap();
    let mock = Mock::new(|request| async move {
        assert_eq!(request.headers["x-tc-token"], "profile-token");
        assert!(
            request.headers["authorization"]
                .to_str()
                .unwrap()
                .contains("Credential=profile-id/")
        );
        Ok(response(
            json!({"Response":{"RequestId":"profile-request"}}),
        ))
    });
    let client = Client::builder()
        .credentials_provider(provider)
        .transport(mock)
        .clock(TestClock::new())
        .build()
        .unwrap();
    let _: ApiResponse<Value> = client
        .execute(
            &Operation::new("cvm", "2017-03-12", "DescribeInstances").unwrap(),
            &json!({}),
        )
        .await
        .unwrap();
}
