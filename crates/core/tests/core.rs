//! Tests the core execution path with fake credentials and a mock transport, without real network access.

use std::{
    collections::VecDeque,
    future::{Future, pending},
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

use bytes::Bytes;
use futures_core::Stream;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tencentcloud_sdk_foundation::{
    ApiResponse, BoxFuture, Client, ClientBuilder, CommonClient, Credentials, Endpoint, Language,
    Operation, RequestOptions, SdkError,
    credentials::ProvideCredentials,
    runtime::Clock,
    transport::{Transport, TransportRequest, TransportResponse},
};

struct FixedClock;
impl Clock for FixedClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(1_551_113_065)
    }
}

type Handler = dyn Fn(TransportRequest) -> BoxFuture<'static, Result<TransportResponse, SdkError>>
    + Send
    + Sync;

#[derive(Clone)]
struct MockTransport {
    handler: Arc<Handler>,
    calls: Arc<AtomicUsize>,
}
impl MockTransport {
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
        self.calls.load(Ordering::Relaxed)
    }
}
impl Transport for MockTransport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        (self.handler)(request)
    }
}

fn builder(mock: MockTransport) -> ClientBuilder {
    Client::builder()
        .credentials(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("test-token")
                .unwrap(),
        )
        .transport(mock)
        .clock(FixedClock)
        .region("ap-shanghai")
}
fn operation() -> Operation {
    Operation::new("cvm", "2017-03-12", "DescribeInstances").unwrap()
}
fn response(status: StatusCode, body: &'static str) -> TransportResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert("x-tc-requestid", HeaderValue::from_static("header-id"));
    TransportResponse::from_bytes(status, headers, body)
}
async fn call(client: &Client) -> Result<ApiResponse<Value>, SdkError> {
    client.execute(&operation(), &json!({})).await
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ListRequest {
    limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ListResponse {
    total_count: u64,
}

#[tokio::test]
async fn typed_round_trip_preserves_exact_payload_and_common_headers() {
    let mock = MockTransport::new(|request| async move {
        assert_eq!(request.method, Method::POST);
        assert_eq!(request.uri, "https://cvm.tencentcloudapi.com/");
        assert_eq!(request.body, br#"{"Limit":1}"#.as_slice());
        assert_eq!(request.headers["x-tc-action"], "DescribeInstances");
        assert_eq!(request.headers["x-tc-version"], "2017-03-12");
        assert_eq!(request.headers["x-tc-timestamp"], "1551113065");
        assert_eq!(request.headers["x-tc-region"], "ap-shanghai");
        assert_eq!(request.headers["host"], "cvm.tencentcloudapi.com");
        assert_eq!(request.headers["x-tc-token"], "test-token");
        assert!(request.headers["x-tc-token"].is_sensitive());
        assert!(request.headers["authorization"].is_sensitive());
        assert!(
            request.headers["authorization"]
                .to_str()
                .unwrap()
                .ends_with("6581ca689bc1f876411008936abf21533fa22ec2cc3ae364dad0685ba61c9623")
        );
        let debug = format!("{request:?}");
        for secret in ["test-id", "test-key", "test-token", "Limit", "Signature="] {
            assert!(!debug.contains(secret));
        }
        Ok(response(
            StatusCode::OK,
            r#"{"Response":{"TotalCount":2,"RequestId":"json-id","FutureField":true}}"#,
        ))
    });
    let client = builder(mock.clone()).build().unwrap();
    let result: ApiResponse<ListResponse> = client
        .execute(
            &operation(),
            &ListRequest {
                limit: 1,
                offset: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(result.data.total_count, 2);
    assert_eq!(result.metadata.request_id.as_deref(), Some("json-id"));
    assert_eq!(result.metadata.status, StatusCode::OK);
    assert_eq!(mock.calls(), 1);
}

#[tokio::test]
async fn local_options_and_per_service_endpoints_do_not_mutate_shared_client() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&requests);
    let mock = MockTransport::new(move |request| {
        captured.lock().unwrap().push(request);
        async { Ok(response(StatusCode::OK, r#"{"Response":{}}"#)) }
    });
    let client = builder(mock)
        .endpoint(
            "cvm",
            Endpoint::https("https://custom.example:8443/").unwrap(),
        )
        .build()
        .unwrap();
    let options = RequestOptions::default()
        .region("ap-guangzhou")
        .language(Language::EnUs)
        .header(
            HeaderName::from_static("x-tc-traceid"),
            HeaderValue::from_static("trace-id"),
        )
        .unwrap();
    let _: ApiResponse<Value> = client
        .execute_with_options(&operation(), &json!({}), &options)
        .await
        .unwrap();
    let common = CommonClient::new(client.clone(), "postgres", "2017-03-12").unwrap();
    let _: ApiResponse<Value> = common
        .call("DescribeDBInstances", &json!({}))
        .await
        .unwrap();
    call(&client).await.unwrap();
    let requests = requests.lock().unwrap();
    assert_eq!(requests[0].uri, "https://custom.example:8443/");
    assert_eq!(requests[0].headers["host"], "custom.example:8443");
    assert!(
        requests[0].headers["authorization"]
            .to_str()
            .unwrap()
            .contains("/cvm/tc3_request")
    );
    assert_eq!(requests[0].headers["x-tc-region"], "ap-guangzhou");
    assert_eq!(requests[0].headers["x-tc-language"], "en-US");
    assert_eq!(requests[0].headers["x-tc-traceid"], "trace-id");
    assert_eq!(requests[1].uri, "https://postgres.tencentcloudapi.com/");
    assert_eq!(requests[1].headers["x-tc-region"], "ap-shanghai");
    assert_eq!(requests[2].headers["x-tc-region"], "ap-shanghai");
    assert!(!requests[2].headers.contains_key("x-tc-language"));
    assert!(!requests[2].headers.contains_key("x-tc-traceid"));
}

#[tokio::test]
async fn service_error_is_preserved_even_on_http_503_and_never_retried() {
    let mock = MockTransport::new(|_| async {
        Ok(response(
            StatusCode::SERVICE_UNAVAILABLE,
            r#"{"Response":{"Error":{"Code":"InternalError","Message":"private-business-data"},"RequestId":"request-503"}}"#,
        ))
    });
    let error = call(&builder(mock.clone()).build().unwrap())
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-business-data"));
    let SdkError::Service(error) = error else {
        panic!("service error must be preserved")
    };
    assert_eq!(error.code, "InternalError");
    assert_eq!(error.message, "private-business-data");
    assert_eq!(error.request_id.as_deref(), Some("request-503"));
    assert_eq!(error.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(mock.calls(), 1);
}

#[tokio::test]
async fn malformed_responses_are_protocol_errors_with_header_request_id() {
    for body in [
        "private-invalid-json",
        "{}",
        r#"{"Response":null}"#,
        r#"{"Response":{"RequestId":42}}"#,
    ] {
        let mock =
            MockTransport::new(move |_| async move { Ok(response(StatusCode::BAD_GATEWAY, body)) });
        let error = call(&builder(mock).build().unwrap()).await.unwrap_err();
        assert!(!format!("{error:?}").contains(body));
        let SdkError::Protocol {
            status, request_id, ..
        } = error
        else {
            panic!("expected a protocol error")
        };
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        assert_eq!(request_id.as_deref(), Some("header-id"));
    }
}

#[tokio::test]
async fn json_response_metadata_falls_back_and_non_200_is_not_success() {
    for body in [
        r#"{"Response":{}}"#,
        r#"{"Response":{"RequestId":null}}"#,
        r#"{"Response":{"RequestId":""}}"#,
    ] {
        let mock = MockTransport::new(move |_| async move { Ok(response(StatusCode::OK, body)) });
        assert_eq!(
            call(&builder(mock).build().unwrap())
                .await
                .unwrap()
                .metadata
                .request_id
                .as_deref(),
            Some("header-id")
        );
    }
    let mock =
        MockTransport::new(|_| async { Ok(response(StatusCode::FOUND, r#"{"Response":{}}"#)) });
    assert!(matches!(
        call(&builder(mock).build().unwrap()).await,
        Err(SdkError::Protocol {
            status: StatusCode::FOUND,
            ..
        })
    ));
}

#[test]
fn rejects_invalid_descriptors_and_protected_headers() {
    for (service, version, action) in [
        ("CVM", "2017-03-12", "Action"),
        ("cvm.evil", "2017-03-12", "Action"),
        ("cvm", "20170312", "Action"),
        ("cvm", "2017-03-12", "Action\n"),
        ("cvm", "2017-03-12", ""),
    ] {
        assert!(Operation::new(service, version, action).is_err());
    }
    for name in [
        "authorization",
        "host",
        "content-type",
        "content-length",
        "transfer-encoding",
        "connection",
        "x-tc-token",
        "x-tc-region",
        "x-tc-version",
        "x-tc-action",
        "x-tc-timestamp",
        "accept-encoding",
    ] {
        assert!(
            RequestOptions::default()
                .header(
                    HeaderName::from_static(name),
                    HeaderValue::from_static("invalid")
                )
                .is_err()
        );
    }
}

struct Chunks(VecDeque<Result<Bytes, SdkError>>);
impl Stream for Chunks {
    type Item = Result<Bytes, SdkError>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.pop_front())
    }
}
struct PendingBody;
impl Stream for PendingBody {
    type Item = Result<Bytes, SdkError>;
    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}

#[tokio::test]
async fn body_limits_apply_to_actual_chunks_not_only_content_length() {
    for declared in [None, Some("1"), Some("1000")] {
        let mock = MockTransport::new(move |_| async move {
            let mut response = response(StatusCode::OK, "");
            if let Some(length) = declared {
                response
                    .headers
                    .insert("content-length", HeaderValue::from_static(length));
            }
            response.body = Box::pin(Chunks(VecDeque::from([
                Ok(Bytes::from_static(b"1234")),
                Ok(Bytes::from_static(b"5678")),
            ])));
            Ok(response)
        });
        assert!(matches!(
            call(&builder(mock).max_response_body(7).build().unwrap()).await,
            Err(SdkError::BodyTooLarge { limit: 7 })
        ));
    }
}

#[tokio::test]
async fn fragmented_body_and_empty_chunks_decode_correctly() {
    let mock = MockTransport::new(|_| async {
        let mut response = response(StatusCode::OK, "");
        response.body = Box::pin(Chunks(VecDeque::from([
            Ok(Bytes::new()),
            Ok(Bytes::from_static(b"{\"Response\":")),
            Ok(Bytes::new()),
            Ok(Bytes::from_static(b"{}}")),
        ])));
        Ok(response)
    });
    assert!(call(&builder(mock).build().unwrap()).await.is_ok());
}

#[tokio::test]
async fn oversized_or_non_object_request_fails_before_transport() {
    let mock =
        MockTransport::new(|_| async { panic!("invalid input must not invoke the transport") });
    let client = builder(mock.clone()).max_request_body(4).build().unwrap();
    let result: Result<ApiResponse<Value>, _> = client
        .execute(&operation(), &json!({"Large":"payload"}))
        .await;
    assert!(matches!(result, Err(SdkError::BodyTooLarge { limit: 4 })));
    let result: Result<ApiResponse<Value>, _> = client.execute(&operation(), &json!([])).await;
    assert!(matches!(result, Err(SdkError::Configuration(_))));
    assert_eq!(mock.calls(), 0);
}

#[tokio::test(start_paused = true)]
async fn timeout_covers_send_and_body_read_and_does_not_retry() {
    let send_pending = MockTransport::new(|_| pending());
    assert!(matches!(
        call(
            &builder(send_pending.clone())
                .timeout(Duration::from_millis(10))
                .build()
                .unwrap()
        )
        .await,
        Err(SdkError::Timeout)
    ));
    assert_eq!(send_pending.calls(), 1);
    let body_pending = MockTransport::new(|_| async {
        let mut response = response(StatusCode::OK, "");
        response.body = Box::pin(PendingBody);
        Ok(response)
    });
    assert!(matches!(
        call(
            &builder(body_pending.clone())
                .timeout(Duration::from_millis(10))
                .build()
                .unwrap()
        )
        .await,
        Err(SdkError::Timeout)
    ));
    assert_eq!(body_pending.calls(), 1);
}

struct PendingCredentials;
impl ProvideCredentials for PendingCredentials {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        Box::pin(pending())
    }
}

#[tokio::test(start_paused = true)]
async fn timeout_covers_credentials_before_network() {
    let mock = MockTransport::new(|_| async {
        panic!("transport must not run before credentials are ready")
    });
    let client = builder(mock.clone())
        .credentials_provider(PendingCredentials)
        .timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    assert!(matches!(call(&client).await, Err(SdkError::Timeout)));
    assert_eq!(mock.calls(), 0);
}

#[tokio::test]
async fn malformed_business_type_and_sse_fail_without_panicking() {
    let mock = MockTransport::new(|_| async {
        Ok(response(
            StatusCode::OK,
            r#"{"Response":{"TotalCount":"private-data","RequestId":"json-id"}}"#,
        ))
    });
    let result: Result<ApiResponse<ListResponse>, _> = builder(mock)
        .build()
        .unwrap()
        .execute(&operation(), &json!({}))
        .await;
    assert!(
        matches!(result, Err(SdkError::Protocol { request_id: Some(id), .. }) if id == "json-id")
    );
    let mock = MockTransport::new(|_| async {
        let mut response = response(StatusCode::OK, "data: private-stream\n\n");
        response.headers.insert(
            "content-type",
            HeaderValue::from_static("text/event-stream; charset=utf-8"),
        );
        Ok(response)
    });
    assert!(matches!(
        call(&builder(mock).build().unwrap()).await,
        Err(SdkError::Protocol { .. })
    ));
}

#[tokio::test]
async fn body_transport_error_preserves_source_but_not_default_debug() {
    let mock = MockTransport::new(|_| async {
        let mut response = response(StatusCode::OK, "");
        response.body = Box::pin(Chunks(VecDeque::from([Err(SdkError::Transport(
            Box::new(std::io::Error::other("private-transport-detail")),
        ))])));
        Ok(response)
    });
    let error = call(&builder(mock.clone()).build().unwrap())
        .await
        .unwrap_err();
    assert!(std::error::Error::source(&error).is_some());
    assert!(!format!("{error:?} {error}").contains("private-transport-detail"));
    assert_eq!(mock.calls(), 1);
}

#[tokio::test]
async fn cloned_client_is_send_sync_and_supports_parallel_calls() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Client>();
    assert_send_sync::<CommonClient>();
    let mock = MockTransport::new(|_| async { Ok(response(StatusCode::OK, r#"{"Response":{}}"#)) });
    let client = builder(mock.clone()).build().unwrap();
    let clone = client.clone();
    let (first, second) = tokio::join!(call(&client), call(&clone));
    assert!(first.is_ok() && second.is_ok());
    let spawned = tokio::spawn(async move { call(&clone).await });
    assert!(spawned.await.unwrap().is_ok());
    assert_eq!(mock.calls(), 3);
}

#[test]
fn configuration_is_explicit_and_limits_must_be_nonzero() {
    assert!(Client::builder().build().is_err());
    let mock =
        MockTransport::new(|_| async { panic!("building a client must not access the network") });
    assert!(
        builder(mock.clone())
            .timeout(Duration::ZERO)
            .build()
            .is_err()
    );
    assert!(builder(mock.clone()).max_request_body(0).build().is_err());
    assert!(builder(mock.clone()).max_response_body(0).build().is_err());
    assert!(builder(mock.clone()).region("invalid\n").build().is_err());
    assert!(
        builder(mock.clone())
            .endpoint(
                "cvm.invalid",
                Endpoint::https("https://example.com").unwrap()
            )
            .build()
            .is_err()
    );
    assert_eq!(mock.calls(), 0);
}

#[cfg(not(feature = "transport-reqwest"))]
#[test]
fn no_features_requires_explicit_transport() {
    assert!(matches!(
        Client::builder()
            .credentials(Credentials::new("test-id", "test-key").unwrap())
            .build(),
        Err(SdkError::Configuration(_))
    ));
}
