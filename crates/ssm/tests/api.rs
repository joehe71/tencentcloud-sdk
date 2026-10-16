//! Offline Secrets Manager tests. Every secret is fake; no cloud I/O is performed.
use http::{HeaderMap, HeaderValue, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    fmt::Debug,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tencentcloud_sdk_core::{
    BoxFuture, Client as CoreClient, Credentials, RequestOptions, SdkError,
    runtime::Clock,
    transport::{Transport, TransportRequest, TransportResponse},
};
use tencentcloud_sdk_ssm::v20190923::{Client, models};

struct FixedClock;
impl Clock for FixedClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(1_551_113_065)
    }
}
struct Mock {
    action: &'static str,
    reply: Value,
    status: StatusCode,
    calls: Arc<AtomicUsize>,
    captured: Arc<Mutex<Option<TransportRequest>>>,
}
impl Transport for Mock {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        assert_eq!(request.method, http::Method::POST);
        assert_eq!(request.uri.to_string(), "https://ssm.tencentcloudapi.com/");
        assert_eq!(request.headers["x-tc-action"], self.action);
        assert_eq!(request.headers["x-tc-version"], "2019-09-23");
        assert_eq!(request.headers["x-tc-timestamp"], "1551113065");
        assert_eq!(request.headers["x-tc-token"], "test-token");
        assert!(
            request.headers["authorization"]
                .to_str()
                .unwrap()
                .contains("/ssm/tc3_request")
        );
        assert!(request.headers["authorization"].is_sensitive());
        *self.captured.lock().unwrap() = Some(request);
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        let response = TransportResponse::from_bytes(self.status, headers, self.reply.to_string());
        Box::pin(async move { Ok(response) })
    }
}
type Capture = Arc<Mutex<Option<TransportRequest>>>;
fn mock_record(
    action: &'static str,
    reply: Value,
    status: StatusCode,
) -> (Client, Arc<AtomicUsize>, Capture) {
    let calls = Arc::new(AtomicUsize::new(0));
    let captured = Arc::new(Mutex::new(None));
    let core = CoreClient::builder()
        .credentials(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("test-token")
                .unwrap(),
        )
        .transport(Mock {
            action,
            reply,
            status,
            calls: calls.clone(),
            captured: captured.clone(),
        })
        .max_request_body(4096)
        .max_response_body(4096)
        .clock(FixedClock)
        .region("ap-shanghai")
        .build()
        .unwrap();
    (Client::new(core), calls, captured)
}
fn mock_client(
    action: &'static str,
    reply: Value,
    status: StatusCode,
) -> (Client, Arc<AtomicUsize>) {
    let (client, calls, _) = mock_record(action, reply, status);
    (client, calls)
}
fn round_trip<T: Serialize + DeserializeOwned + Default + Debug + Clone>(
    sample: &str,
    nulls: &str,
) {
    let expected: Value = serde_json::from_str(sample).unwrap();
    let model: T = serde_json::from_str(sample).unwrap();
    assert_eq!(serde_json::to_value(model.clone()).unwrap(), expected);
    assert!(!format!("{model:?}").contains("test-marker"));
    assert_eq!(serde_json::to_value(T::default()).unwrap(), json!({}));
    assert_eq!(
        serde_json::to_value(serde_json::from_str::<T>(nulls).unwrap()).unwrap(),
        json!({})
    );
}
include!("generated/checks.rs");

#[tokio::test]
async fn create_and_binary_version_preserve_exact_values_without_debug_leaks() {
    let input = models::CreateSecretRequest {
        secret_name: Some("fake-secret-name".into()),
        version_id: Some("v1".into()),
        secret_string: Some("{\"password\":\"fake-测试\\value\"}".into()),
        tags: Some(vec![models::Tag {
            tag_key: Some("env".into()),
            tag_value: Some("test".into()),
        }]),
        ..Default::default()
    };
    let (client, calls, captured) = mock_record(
        "CreateSecret",
        json!({"Response":{"SecretName":"fake-secret-name","RequestId":"create-1"}}),
        StatusCode::OK,
    );
    client.create_secret(&input).await.unwrap();
    let wire = captured.lock().unwrap().clone().unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&wire.body).unwrap(),
        json!({
            "SecretName":"fake-secret-name", "VersionId":"v1", "SecretString":"{\"password\":\"fake-测试\\value\"}",
            "Tags":[{"TagKey":"env","TagValue":"test"}]
        })
    );
    assert!(!format!("{input:?} {wire:?} {client:?}").contains("fake-secret-name"));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    let _shared = client.core().clone();
    let input = models::PutSecretValueRequest {
        secret_name: Some("fake-secret-name".into()),
        version_id: Some("v2".into()),
        secret_binary: Some("AP8BAg==".into()),
        secret_string: None,
    };
    let (client, calls, captured) = mock_record(
        "PutSecretValue",
        json!({"Response":{"RequestId":"put-1"}}),
        StatusCode::OK,
    );
    client.put_secret_value(&input).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&captured.lock().unwrap().as_ref().unwrap().body).unwrap(),
        json!({"SecretName":"fake-secret-name","VersionId":"v2","SecretBinary":"AP8BAg=="})
    );
    assert!(!format!("{input:?}").contains("AP8BAg=="));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn secret_reads_return_plaintext_explicitly_without_implicit_cache_or_logging() {
    let (client, calls) = mock_client(
        "GetSecretValue",
        json!({"Response":{"SecretName":"fake-name","VersionId":"SSM_Current","SecretString":"fake-sensitive-value","SecretBinary":null,"RequestId":"read-1","NewField":true}}),
        StatusCode::OK,
    );
    let input = models::GetSecretValueRequest {
        secret_name: Some("fake-name".into()),
        version_id: Some("SSM_Current".into()),
        ..Default::default()
    };
    let reply = client.clone().get_secret_value(&input).await.unwrap();
    assert_eq!(
        reply.data.secret_string.as_deref(),
        Some("fake-sensitive-value")
    );
    assert!(reply.data.secret_binary.is_none());
    assert_eq!(reply.metadata.request_id.as_deref(), Some("read-1"));
    assert!(!format!("{:?}", reply.data).contains("fake-sensitive-value"));
    client.get_secret_value(&input).await.unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[tokio::test]
async fn ssh_private_keys_keep_base64_and_large_project_ids_but_redact_debug() {
    let (client, calls) = mock_client(
        "GetSSHKeyPairValue",
        json!({"Response":{"SSHKeyID":"skey-test","PublicKey":"ZmFrZS1wdWJsaWM=","PrivateKey":"ZmFrZS1wcml2YXRl","ProjectID":9_007_199_254_740_993_i64,"RequestId":"ssh-1"}}),
        StatusCode::OK,
    );
    let reply = client
        .get_ssh_key_pair_value(&models::GetSshKeyPairValueRequest::default())
        .await
        .unwrap();
    assert_eq!(reply.data.private_key.as_deref(), Some("ZmFrZS1wcml2YXRl"));
    assert_eq!(reply.data.project_id, Some(9_007_199_254_740_993));
    assert!(!format!("{:?}", reply.data).contains("ZmFrZS1wcml2YXRl"));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn deletion_parameters_are_explicit_and_failures_never_replay_or_rotate() {
    let input = models::DeleteSecretRequest {
        secret_name: Some("fake-name".into()),
        recovery_window_in_days: Some(7),
        clean_ssh_key: Some(false),
        delete_mode: Some(0),
    };
    let (client, calls, captured) = mock_record(
        "DeleteSecret",
        json!({"Response":{"Error":{"Code":"ResourceUnavailable","Message":"fake-sensitive-value"},"RequestId":"delete-1"}}),
        StatusCode::SERVICE_UNAVAILABLE,
    );
    let error = client.delete_secret(&input).await.unwrap_err();
    let SdkError::Service(service) = &error else {
        panic!("expected service error")
    };
    assert_eq!(service.code, "ResourceUnavailable");
    assert_eq!(service.message, "fake-sensitive-value");
    assert_eq!(service.request_id.as_deref(), Some("delete-1"));
    assert!(!format!("{error:?} {error}").contains("fake-sensitive-value"));
    assert_eq!(
        serde_json::from_slice::<Value>(&captured.lock().unwrap().as_ref().unwrap().body).unwrap(),
        json!({"SecretName":"fake-name","RecoveryWindowInDays":7,"CleanSSHKey":false,"DeleteMode":0})
    );
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn explicit_list_pages_and_options_do_not_mutate_the_shared_core() {
    let input = models::ListSecretsRequest {
        offset: Some(10),
        limit: Some(5),
        search_secret_name: Some("fake-prefix".into()),
        ..Default::default()
    };
    let (client, calls, captured) = mock_record(
        "ListSecrets",
        json!({"Response":{"TotalCount":20,"SecretMetadatas":[],"RequestId":"list-1"}}),
        StatusCode::OK,
    );
    client
        .execute_with_options(&input, &RequestOptions::default().region("ap-guangzhou"))
        .await
        .unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(
        captured.lock().unwrap().as_ref().unwrap().headers["x-tc-region"],
        "ap-guangzhou"
    );
    client.list_secrets(&input).await.unwrap();
    let wire = captured.lock().unwrap().clone().unwrap();
    assert_eq!(wire.headers["x-tc-region"], "ap-shanghai");
    assert_eq!(
        serde_json::from_slice::<Value>(&wire.body).unwrap(),
        json!({"Offset":10,"Limit":5,"SearchSecretName":"fake-prefix"})
    );
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[tokio::test]
async fn bounded_payloads_zero_budget_and_malformed_values_keep_core_failures() {
    let (client, calls) = mock_client("CreateSecret", json!({"Response":{}}), StatusCode::OK);
    let input = models::CreateSecretRequest {
        secret_string: Some("fake".repeat(2048)),
        ..Default::default()
    };
    assert!(matches!(
        client.create_secret(&input).await,
        Err(SdkError::BodyTooLarge { .. })
    ));
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    let input = models::GetSecretValueRequest::default();
    let (client, calls) = mock_client(
        "GetSecretValue",
        json!({"Response":{"SecretString":{"Unexpected":"fake-sensitive-value"},"RequestId":"invalid-1"}}),
        StatusCode::OK,
    );
    let error = client.get_secret_value(&input).await.unwrap_err();
    assert!(matches!(error, SdkError::Protocol { .. }));
    assert!(!format!("{error:?}").contains("fake-sensitive-value"));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    let error = client
        .execute_with_options(&input, &RequestOptions::default().timeout(Duration::ZERO))
        .await
        .unwrap_err();
    assert!(matches!(error, SdkError::Configuration(_)));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    let (client, calls) = mock_client(
        "GetSecretValue",
        json!({"Response":{"SecretString":"fake".repeat(2048)}}),
        StatusCode::OK,
    );
    assert!(matches!(
        client.get_secret_value(&input).await,
        Err(SdkError::BodyTooLarge { .. })
    ));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn encryption_parameters_remain_explicit_wire_values_without_local_crypto() {
    let input = models::GetSecretValueRequest {
        secret_name: Some("fake-name".into()),
        version_id: Some("v1".into()),
        encryption_public_key: Some("fake-public-key".into()),
        encryption_algorithm: Some("fake-algorithm".into()),
    };
    let (client, calls, captured) = mock_record(
        "GetSecretValue",
        json!({"Response":{"SecretString":"opaque-service-value","RequestId":"enc-1"}}),
        StatusCode::OK,
    );
    let reply = client.get_secret_value(&input).await.unwrap();
    assert_eq!(
        reply.data.secret_string.as_deref(),
        Some("opaque-service-value")
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&captured.lock().unwrap().as_ref().unwrap().body).unwrap(),
        json!({"SecretName":"fake-name","VersionId":"v1","EncryptionPublicKey":"fake-public-key","EncryptionAlgorithm":"fake-algorithm"})
    );
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}
