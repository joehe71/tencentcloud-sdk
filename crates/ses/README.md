# Tencent Cloud SES

Typed asynchronous API 3.0 calls for SES **2020-10-02**, exposed as `tencentcloud_sdk::ses::v20201002` by the facade's optional `ses` feature. The standalone crate is `tencentcloud-sdk-ses`. Packages are not published yet.

## Use

Enable `ses` plus `transport-reqwest` in the facade, or inject an application-owned core transport. The product adds no runtime dependencies beyond core and serde; it does not depend on COS. A product client is a cloneable view over an existing core client. Clones share credentials, refresh cache, endpoint configuration and the HTTP pool.

```rust,no_run
use tencentcloud_sdk_core::{Client as CoreClient, SdkError};
use tencentcloud_sdk_ses::v20201002::{Client, models::{SendEmailRequest, Template}};

async fn send(core: CoreClient) -> Result<(), SdkError> {
    let client = Client::new(core);
    let request = SendEmailRequest {
        from_email_address: Some("sender@example.invalid".into()),
        destination: Some(vec!["recipient@example.invalid".into()]),
        subject: Some("Example".into()),
        template: Some(Template {
            template_id: Some(123), template_data: Some(r#"{"name":"example"}"#.into()),
        }),
        ..Default::default()
    };
    // Real sends need explicit application authorization and a verified SES identity.
    let reply = client.send_email(&request).await?;
    let _message_id = reply.data.message_id;
    let _request_id = reply.metadata.request_id;
    Ok(())
}
```

Use `client.execute_with_options(&request, &RequestOptions::default().region("ap-hongkong"))` for per-call region, language, headers and timeout overrides. Select a region supported by your SES account; the wrapper does not choose one silently. The core endpoint override key is `ses`; the default host is `ses.tencentcloudapi.com` and the signed version is `2020-10-02`.

## Operations

All **36 actions in the referenced Java snapshot** have typed methods and request/response bindings:

- Sending: `send_email`, `batch_send_email`.
- Identities: `create_email_identity`, `get_email_identity`, `list_email_identities`, `update_email_identity`, `delete_email_identity`.
- Senders: `create_email_address`, `list_email_address`, `delete_email_address`, `update_email_smtp_password`.
- Templates: `create_email_template`, `get_email_template`, `list_email_templates`, `update_email_template`, `delete_email_template`.
- Receivers: `create_receiver`, `create_receiver_detail`, `create_receiver_detail_with_data`, `list_receivers`, `list_receiver_details`, `delete_receiver`.
- Reports: `get_abuse_report`, `get_send_email_status`, `get_statistics_report`, `list_send_tasks`.
- Blacklists: `create_custom_blacklist`, `list_custom_blacklist`, `update_custom_blacklist`, `delete_custom_blacklist`, `list_black_email_address`, `delete_blacklist`.
- Unsubscribe configuration: `create_address_unsubscribe_config`, `list_address_unsubscribe_config`, `update_address_unsubscribe_config`, `delete_address_unsubscribe_config`.

`execute` and `execute_with_options` accept a sealed typed `Request`, so the action always selects the correct response type. Unknown future actions remain accessible through the existing core generic API; no claim of support for newer SES actions is implied.

## Wire semantics and safety

- 93 explicit Rust models preserve exact JSON field names, including `TemplateID`, `TaskID`, `DKIMOption` and the original action `UpdateEmailSmtpPassWord`. Rust names normalize acronyms, `Blacklist` and `Password` rather than copying Java accessors.
- Optional fields use `Option<T>` and serialize `None` as an omitted field, not JSON null. Responses accept absent/null fields and ignore unknown fields for forward compatibility. Required-field, recipient, template and business constraints remain server-validated: `Default` constructs an empty model, not a valid mail request. There is no builder claiming compile-time business validation.
- Integers remain integer JSON values (`i64` for Java Long), including values larger than JavaScript's exact integer range. Strings, encoded content and template JSON are passed verbatim. Attachments must already contain the service-required Base64 content; the SDK does not read files, fetch remote URLs or encode attachments implicitly.
- The API's mail-body and attachment limits still apply. Configure `CoreClient::builder().max_request_body(...)` to an appropriate smaller application limit; the generic core's default buffer limit is not a promise that SES accepts a payload of that size.
- Models implement redacted Debug without any field values. Explicit serialization and accessors intentionally expose application data: do not log request/response JSON, plaintext email addresses or SMTP passwords. Model `RequestId` fields and core response metadata both retain the official identifier.
- Calls use the shared core TC3 signature, token handling, deadlines and body bounds. Service errors are returned unchanged, preserving official code, message and `RequestId`; default formatting omits the message. Side effects are sent **once**, including email delivery and receiver/template changes.
- List operations return exactly one page. Manage Limit/Offset, totals and subsequent calls explicitly. There are no automatic sends, retries, pagination, task polling, delivery guarantees or unsubscribe business workflows.

## Offline verification and protocol reference

Tests invoke all 36 typed methods and verify exact actions, service/version/token headers, signing scope and response types. Every model is round-tripped with all fields populated and with JSON null, and its Debug redaction is checked. Independent hand-written fixtures verify nested templates, attachments, recipient lists, large integers, pagination options, SMTP passwords, malformed typed responses and unchanged service errors without replay. No message is sent to a real account.

Checked-in files compile without Java, Python, network access, or generation at build time. Protocol metadata references Tencent Cloud Java SDK 3.2.40, `ses/v20201002`. Maintain the Rust models, typed bindings, schema snapshot, and fixtures together when updating the supported protocol.

```bash
cargo fmt --all
cargo test -p tencentcloud-sdk-ses
bash scripts/check-coverage.sh
```

The reference contributes annotated field names/types and action identifiers, not Java inheritance, behavior, or comments. The checked-in schema records the precise wire snapshot; model fixtures supplement, rather than replace, independent hand-written tests. Attribution and the upstream Apache-2.0 license are retained in `THIRD_PARTY_NOTICES.md` and `LICENSE`. See [official SES API documentation](https://cloud.tencent.com/document/product/1288/51034).
