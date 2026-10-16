//! Offline SES protocol tests; no mail is sent and no cloud endpoint is contacted.
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
use tencentcloud_sdk_foundation::{
    BoxFuture, Client as CoreClient, Credentials, Language, RequestOptions, SdkError,
    runtime::Clock,
    transport::{Transport, TransportRequest, TransportResponse},
};
use tencentcloud_sdk_ses::v20201002::{Client, models};

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
        assert_eq!(request.uri.to_string(), "https://ses.tencentcloudapi.com/");
        assert_eq!(request.headers["x-tc-action"], self.action);
        assert_eq!(request.headers["x-tc-version"], "2020-10-02");
        assert_eq!(request.headers["x-tc-timestamp"], "1551113065");
        assert_eq!(request.headers["x-tc-token"], "test-token");
        assert!(
            request.headers["authorization"]
                .to_str()
                .unwrap()
                .contains("/ses/tc3_request")
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
async fn sends_exact_nested_template_attachment_and_recipient_fields_once() {
    let input = models::SendEmailRequest {
        from_email_address: Some("Sender <sender@example.invalid>".into()),
        destination: Some(vec!["to@example.invalid".into()]),
        cc: Some(vec![]),
        bcc: Some(vec!["hidden@example.invalid".into()]),
        subject: Some("测试 \"quoted\" subject".into()),
        template: Some(models::Template {
            template_id: Some(9_007_199_254_740_993),
            template_data: Some("{\"name\":\"测试\"}".into()),
        }),
        attachments: Some(vec![models::Attachment {
            file_name: Some("hello.txt".into()),
            content: Some("aGVsbG8=".into()),
            file_url: None,
        }]),
        ..Default::default()
    };
    let (client, calls, captured) = mock_record(
        "SendEmail",
        json!({"Response":{"MessageId":"mail-1","RequestId":"request-1","FutureField":123}}),
        StatusCode::OK,
    );
    let reply = client.clone().send_email(&input).await.unwrap();
    assert_eq!(reply.data.message_id.as_deref(), Some("mail-1"));
    assert_eq!(reply.metadata.request_id.as_deref(), Some("request-1"));
    let request = captured.lock().unwrap().clone().unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&request.body).unwrap(),
        json!({
            "FromEmailAddress":"Sender <sender@example.invalid>","Destination":["to@example.invalid"],"Cc":[],"Bcc":["hidden@example.invalid"],
            "Subject":"测试 \"quoted\" subject", "Template":{"TemplateID":9_007_199_254_740_993_i64,"TemplateData":"{\"name\":\"测试\"}"},
            "Attachments":[{"FileName":"hello.txt","Content":"aGVsbG8="}]
        })
    );
    for value in [
        "sender@example.invalid",
        "hidden@example.invalid",
        "aGVsbG8=",
    ] {
        assert!(!format!("{input:?} {request:?} {client:?}").contains(value));
    }
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    let _core_view = client.core().clone();
}

#[tokio::test]
async fn per_call_options_do_not_change_the_shared_client() {
    let (client, calls, captured) = mock_record(
        "ListEmailTemplates",
        json!({"Response":{"TemplatesMetadata":null,"RequestId":"page-1"}}),
        StatusCode::OK,
    );
    let request = models::ListEmailTemplatesRequest {
        limit: Some(10),
        offset: Some(20),
    };
    let options = RequestOptions::default()
        .region("ap-guangzhou")
        .language(Language::EnUs);
    let reply = client
        .execute_with_options(&request, &options)
        .await
        .unwrap();
    assert!(reply.data.templates_metadata.is_none());
    assert_eq!(
        captured.lock().unwrap().as_ref().unwrap().headers["x-tc-region"],
        "ap-guangzhou"
    );
    assert_eq!(
        captured.lock().unwrap().as_ref().unwrap().headers["x-tc-language"],
        "en-US"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 1); // No implicit pagination.
    client.list_email_templates(&request).await.unwrap();
    let wire = captured.lock().unwrap().clone().unwrap();
    assert_eq!(wire.headers["x-tc-region"], "ap-shanghai");
    assert_eq!(
        serde_json::from_slice::<Value>(&wire.body).unwrap(),
        json!({"Limit":10,"Offset":20})
    );
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    let error = client
        .execute_with_options(&request, &RequestOptions::default().timeout(Duration::ZERO))
        .await
        .unwrap_err();
    assert!(matches!(error, SdkError::Configuration(_)));
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[tokio::test]
async fn service_errors_preserve_metadata_without_replaying_or_leaking_mail() {
    let (client, calls) = mock_client(
        "SendEmail",
        json!({"Response":{"Error":{"Code":"RequestLimitExceeded","Message":"private@example.invalid"},"RequestId":"failed-request"}}),
        StatusCode::TOO_MANY_REQUESTS,
    );
    let error = client
        .send_email(&models::SendEmailRequest::default())
        .await
        .unwrap_err();
    let SdkError::Service(service) = &error else {
        panic!("expected service error")
    };
    assert_eq!(service.code, "RequestLimitExceeded");
    assert_eq!(service.request_id.as_deref(), Some("failed-request"));
    assert_eq!(service.message, "private@example.invalid");
    assert!(!format!("{error:?} {error}").contains("private@example.invalid"));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn malformed_typed_data_and_smtp_secrets_keep_core_contracts() {
    let (client, calls) = mock_client(
        "SendEmail",
        json!({"Response":{"MessageId":17,"RequestId":"bad-data"}}),
        StatusCode::OK,
    );
    assert!(matches!(
        client
            .send_email(&models::SendEmailRequest::default())
            .await,
        Err(SdkError::Protocol { .. })
    ));
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    let input = models::UpdateEmailSmtpPasswordRequest {
        password: Some("fake-smtp-secret".into()),
        email_address: Some("sender@example.invalid".into()),
    };
    let (client, _, captured) = mock_record(
        "UpdateEmailSmtpPassWord",
        json!({"Response":{"RequestId":"smtp-1"}}),
        StatusCode::OK,
    );
    client.update_email_smtp_password(&input).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&captured.lock().unwrap().as_ref().unwrap().body).unwrap(),
        json!({"Password":"fake-smtp-secret","EmailAddress":"sender@example.invalid"})
    );
    assert!(!format!("{input:?}").contains("fake-smtp-secret"));
}
