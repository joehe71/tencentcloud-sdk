//! Offline shared instance-role tests; all credentials and service responses are fake.
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering},
};

use http::{HeaderMap, Method, StatusCode};
use serde_json::json;

use crate::{
    BoxFuture, Client, SdkError, cos,
    credentials::{
        CvmRoleCredentialsProvider, InstanceRoleCredentialsProvider, ProvideCredentials,
        RoleProviderOptions,
    },
    runtime::Clock,
    ssm::v20190923::{self as ssm, models::GetSecretValueRequest},
    transport::{Transport, TransportRequest, TransportResponse},
};

const NOW: i64 = 1_700_000_000;
const METADATA_HOST: &str = "metadata.tencentyun.com";
const ROLE_PATH: &str = "/latest/meta-data/cam/security-credentials/";

#[derive(Clone)]
struct TestClock(Arc<AtomicI64>);
impl Clock for TestClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

#[derive(Default)]
struct Mock {
    discoveries: AtomicUsize,
    issues: AtomicUsize,
    business: AtomicUsize,
    denied: AtomicBool,
}
impl Mock {
    fn respond(&self, request: &TransportRequest) -> TransportResponse {
        if request.uri.host() == Some(METADATA_HOST) {
            assert_eq!(request.uri.scheme_str(), Some("http"));
            assert_eq!(request.method, Method::GET);
            assert!(request.headers.is_empty());
            assert!(request.body.is_empty());
            if request.uri.path() == ROLE_PATH {
                self.discoveries.fetch_add(1, Ordering::SeqCst);
                return TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    "instance-role\n",
                );
            }
            assert_eq!(request.uri.path(), format!("{ROLE_PATH}instance-role"));
            let generation = self.issues.fetch_add(1, Ordering::SeqCst) + 1;
            if self.denied.load(Ordering::SeqCst) {
                return TransportResponse::from_bytes(
                    StatusCode::FORBIDDEN,
                    HeaderMap::new(),
                    "metadata unavailable",
                );
            }
            return TransportResponse::from_bytes(
                StatusCode::OK,
                HeaderMap::new(),
                json!({
                    "Code": "Success",
                    "TmpSecretId": format!("instance-id-{generation}"),
                    "TmpSecretKey": format!("instance-key-{generation}"),
                    "Token": format!("instance-token-{generation}"),
                    "ExpiredTime": NOW + 3600 * i64::try_from(generation).unwrap(),
                })
                .to_string(),
            );
        }
        self.business.fetch_add(1, Ordering::SeqCst);
        assert_eq!(request.uri.scheme_str(), Some("https"));
        let generation = self.issues.load(Ordering::SeqCst);
        let authorization = &request.headers["authorization"];
        assert!(authorization.is_sensitive());
        let auth = authorization.to_str().unwrap();
        let (token_header, body) = match request.uri.host().unwrap() {
            "ssm.tencentcloudapi.com" => {
                assert_eq!(request.method, Method::POST);
                assert_eq!(request.headers["x-tc-action"], "GetSecretValue");
                assert_eq!(request.headers["x-tc-version"], "2019-09-23");
                assert!(auth.contains(&format!("Credential=instance-id-{generation}/")));
                assert!(auth.contains("/ssm/tc3_request"));
                (
                    "x-tc-token",
                    json!({"Response": {"SecretString": "fake-application-secret", "RequestId": "ssm-request"}}).to_string(),
                )
            }
            "examplebucket-1250000000.cos.ap-shanghai.myqcloud.com" => {
                assert_eq!(request.method, Method::PUT);
                assert_eq!(request.uri.path(), "/telemetry.json");
                assert_eq!(request.body.as_ref(), b"{}");
                assert!(auth.contains(&format!("q-ak=instance-id-{generation}&")));
                assert!(auth.contains("x-cos-security-token"));
                ("x-cos-security-token", String::new())
            }
            other => panic!("unexpected endpoint: {other}"),
        };
        assert_eq!(
            request.headers[token_header].to_str().unwrap(),
            format!("instance-token-{generation}")
        );
        assert!(request.headers[token_header].is_sensitive());
        TransportResponse::from_bytes(StatusCode::OK, HeaderMap::new(), body)
    }
}
impl Transport for Mock {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        Box::pin(async move {
            // Force concurrent SSM/COS calls to overlap during metadata acquisition.
            tokio::task::yield_now().await;
            Ok(self.respond(&request))
        })
    }
}

fn clients(
    role: Option<String>,
) -> (
    ssm::Client,
    cos::Client,
    InstanceRoleCredentialsProvider,
    Arc<Mock>,
    TestClock,
) {
    let transport = Arc::new(Mock::default());
    let clock = TestClock(Arc::new(AtomicI64::new(NOW)));
    let provider = InstanceRoleCredentialsProvider::new(
        role,
        RoleProviderOptions::new(transport.clone()).clock(clock.clone()),
    )
    .unwrap();
    // The historical CVM name is the same type, not a separate cache or wrapper.
    let legacy: CvmRoleCredentialsProvider = provider.clone();
    let core = Client::builder()
        .credentials_provider(legacy)
        .shared_transport(transport.clone())
        .clock(clock.clone())
        .region("ap-shanghai")
        .build()
        .unwrap();
    let cos = cos::Client::builder("examplebucket-1250000000", "ap-shanghai")
        .credentials_provider(provider.clone())
        .shared_transport(transport.clone())
        .clock(clock.clone())
        .build()
        .unwrap();
    assert_eq!(transport.issues.load(Ordering::SeqCst), 0);
    assert_eq!(transport.discoveries.load(Ordering::SeqCst), 0);
    (ssm::Client::new(core), cos, provider, transport, clock)
}

fn secret_request() -> GetSecretValueRequest {
    GetSecretValueRequest {
        secret_name: Some("application-config".into()),
        version_id: Some("SSM_Current".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn ssm_and_cos_share_instance_credentials_and_refresh_together() {
    let (ssm, cos, provider, transport, clock) = clients(None);
    let secret = ssm.get_secret_value(&secret_request()).await.unwrap();
    assert_eq!(
        secret.data.secret_string.as_deref(),
        Some("fake-application-secret")
    );
    cos.put_object("telemetry.json", "{}".into(), None)
        .await
        .unwrap();
    let first = provider.provide_credentials().await.unwrap();
    assert_eq!(transport.issues.load(Ordering::SeqCst), 1);
    assert_eq!(transport.discoveries.load(Ordering::SeqCst), 1);

    clock.0.store(NOW + 3000, Ordering::SeqCst);
    let request = secret_request();
    let (secret, object) = tokio::join!(
        ssm.get_secret_value(&request),
        cos.put_object("telemetry.json", "{}".into(), None),
    );
    secret.unwrap();
    object.unwrap();
    assert_eq!(transport.issues.load(Ordering::SeqCst), 2);
    assert_eq!(transport.discoveries.load(Ordering::SeqCst), 2);
    assert_eq!(transport.business.load(Ordering::SeqCst), 4);
    assert!(!Arc::ptr_eq(
        &first,
        &provider.provide_credentials().await.unwrap()
    ));
}

#[tokio::test]
async fn explicit_role_skips_discovery_and_failed_refresh_stops_both_services() {
    let (ssm, cos, provider, transport, clock) = clients(Some("instance-role".into()));
    provider.provide_credentials().await.unwrap();
    assert_eq!(transport.discoveries.load(Ordering::SeqCst), 0);
    transport.denied.store(true, Ordering::SeqCst);
    clock.0.store(NOW + 3000, Ordering::SeqCst);
    assert!(ssm.get_secret_value(&secret_request()).await.is_err());
    assert!(
        cos.put_object("telemetry.json", "{}".into(), None)
            .await
            .is_err()
    );
    assert_eq!(transport.issues.load(Ordering::SeqCst), 2);
    assert_eq!(transport.business.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn invalid_instance_role_is_rejected_without_metadata_access() {
    let transport = Arc::new(Mock::default());
    assert!(
        InstanceRoleCredentialsProvider::new(
            Some("../another-role".into()),
            RoleProviderOptions::new(transport.clone()),
        )
        .is_err()
    );
    assert_eq!(transport.issues.load(Ordering::SeqCst), 0);
    assert_eq!(transport.discoveries.load(Ordering::SeqCst), 0);
}
