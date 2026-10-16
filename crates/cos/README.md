# Tencent Cloud COS

An asynchronous COS **XML REST API** client, separate from API 3.0 JSON and TC3. The package is not published yet. The facade exposes it as `tencentcloud_sdk::cos` when `cos` is enabled; direct users can depend on `tencentcloud-sdk-cos`.

## Construction

```rust,no_run
use std::sync::Arc;
use tencentcloud_sdk_core::{Credentials, Endpoint, credentials::StaticCredentialsProvider,
    transport::Transport};
use tencentcloud_sdk_cos::{Client, Error};

fn build_client(transport: Arc<dyn Transport>) -> Result<Client, Error> {
    // Fake values illustrate wiring only; applications must supply an authorized provider.
    let provider = StaticCredentialsProvider::new(Credentials::new("test-id", "test-key")?);
    Client::builder("examplebucket-1250000000", "ap-shanghai")
        .credentials_provider(provider)
        .shared_transport(transport)
        .build()
}
```

Construction performs no credential lookup or network I/O. Reuse `RoleProviderOptions` and the same transport to supply a refreshing CVM, `AssumeRole` or TKE OIDC provider; COS and STS use the same core credentials type. No permanent AK/SK is required when Pod identity is configured.

With the facade:

```toml
[dependencies]
tencentcloud-sdk = { path = "../tencentcloud-sdk", default-features = false, features = ["cos", "transport-reqwest"] }
```

`transport-reqwest` supplies the default pool-backed transport. It is independent of the COS product feature. Without it, explicitly inject a transport. The default bucket endpoint is `https://{bucket}.cos.{region}.myqcloud.com/`; GET Service uses `https://service.cos.myqcloud.com/`. Trusted endpoint overrides are independent; HTTP overrides require numeric loopback roots for offline tests.

## Supported operations

| API | Rust method |
| --- | --- |
| PUT / GET / HEAD / DELETE Object | `put_object`, `get_object`, `head_object`, `delete_object` |
| Range GET Object | `get_object_range` |
| PUT Object - Copy | `copy_object` |
| GET Bucket | `list_objects` |
| GET Service | `list_buckets` |
| PUT / HEAD / DELETE Bucket | `create_bucket`, `head_bucket`, `delete_bucket` |
| GET Bucket location | `get_bucket_location` |
| Initiate / Upload Part / List Parts / Complete / Abort Multipart Upload | `initiate_multipart_upload`, `upload_part`, `list_parts`, `complete_multipart_upload`, `abort_multipart_upload` |
| Presigned GET / HEAD / PUT / DELETE | `presign` |

Lists return **one page**. Pass `next_marker` or `next_part_number_marker` explicitly; no automatic requests or hidden retries occur. `Response<T>` retains HTTP status, COS `RequestId` and headers, including `ETag`, version and metadata. Entity tags are opaque, not assumed to be MD5 checksums. Keys and query inputs are literal UTF-8, not pre-encoded.

```rust,no_run
use bytes::Bytes;
use tencentcloud_sdk_cos::{Client, Error, models::ListObjectsOptions};

async fn example(client: &Client) -> Result<(), Error> {
    client.put_object("folder/example.txt", Bytes::from_static(b"hello"), Some("text/plain")).await?;
    let object = client.get_object("folder/example.txt").await?;
    assert_eq!(object.data, "hello");
    let page = client.list_objects(&ListObjectsOptions {
        prefix: Some("folder/".into()), max_keys: Some(100), ..Default::default()
    }).await?;
    let _next_page_marker = page.data.next_marker;
    Ok(())
}
```

These examples are compiled, not run. Actual cloud use requires the application's explicit authorization and appropriate CAM policy. The repository's tests only use mocks, fake credentials and numeric loopback HTTP.

## Signing, integrity and limits

- Implements COS HMAC-SHA1 authentication with canonical query/header ordering, one-time path decoding, RFC 3986 component encoding and the same credential snapshot for signature and session token. SHA-1 is used only because COS mandates this protocol, not as a general-purpose security recommendation.
- Temporary tokens use sensitive `x-cos-security-token` headers. Presigning puts tokens in the signed query. Signed URLs are bearer capabilities; `PresignedUrl` is redacted in Debug and offers only explicit `as_str()` access. Never log or persist the resulting string in diagnostics.
- Requested URL lifetimes must be 1 second through 7 days, but temporary credentials may expire sooner. The application must choose a lifetime within its identity's validity; the SDK cannot promise validity beyond STS expiration. Ordinary requests use a 15-minute signature window.
- Uploads, parts and XML POST/PUT bodies include Content-MD5 and exact Content-Length. Downloads do not automatically validate CRC64 or `ETag` checksums; HTTPS provides transport integrity. No download checksum-verification guarantee is claimed.
- Default request and response buffers are each limited to 32 MiB, including actual streamed response chunks, not trusted Content-Length. Override explicitly when appropriate. Downloads return buffered `Bytes`, not a file stream; use explicit byte ranges for bounded reads of large objects. Upload parts are also buffered; there is no automatic multipart transfer manager or background abort.
- Default call budget: 60 seconds, covering credentials, request preparation, send and response reads. Synchronous checksum/XML work is checked at stage boundaries rather than preempted. Client clones share configuration, provider and transport. Dropping a request cancels it without detached business tasks.
- XML responses are bounded, checked for expected roots and parsed without external entities or DTD acceptance. HTTP 200 completion errors are parsed as COS service failures, preserving error codes, messages and `RequestId`. Ordinary object downloads are never interpreted as control XML, even if their bytes look like an Error document.
- Side effects are sent once. Redirects, environment proxies and automatic replay are disabled by the default transport; custom transports must honor the same contract. No implicit bucket cleanup, retry, credential-source fallback or region failover is performed.
- Full bucket names must include APPID. Keys may be up to 850 UTF-8 bytes; control characters and `.`/`..` path segments are rejected to prevent transport URL normalization from silently changing the target key.

## Explicit limitations

This initial product crate does not implement the entire COS API: version-specific operations, ACL/policy/lifecycle/tagging configuration, object metadata options, CORS, multipart copy, List Multipart Uploads, batch delete, automatic pagination, CRC64 verification, local-file streaming and concurrent transfer management are not provided. Existing raw transport extensibility is not advertised as support for these operations.

## Verification

Offline tests check every supported operation's method, path, query, signed headers, multipart XML escaping, explicit pagination and response fields. A fixed signature vector is calculated independently with Python hashlib/hmac. Additional tests cover token handling, malformed XML, DTD rejection, HTTP 200 embedded errors, redaction, actual body bounds, deadline phases, non-replay, and a real reqwest loopback exchange preserving encoded keys, tokens, checksums and connection reuse.

Run `cargo test --workspace --all-features --locked`, strict Clippy, and `bash scripts/check-coverage.sh` (production line coverage >=90%). No cloud integration is claimed.

Protocol references: [COS authentication](https://cloud.tencent.com/document/product/436/7778), [COS API](https://cloud.tencent.com/document/product/436/7751), [official Go signing implementation](https://github.com/tencentyun/cos-go-sdk-v5/blob/master/auth.go).
