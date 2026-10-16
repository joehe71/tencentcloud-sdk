# Tencent Cloud Rust SDK

An idiomatic Rust SDK for Tencent Cloud API 3.0, with one facade crate and separate shared-core and product crates. The Java SDK is a protocol and model reference only; its class hierarchy and reflection-based calls are not ported.

The workspace includes the `tencentcloud-sdk-foundation` package (Rust library name `tencentcloud_sdk_foundation`) and opt-in COS, SES, and SSM crates. Other products, automatic pagination, SSE, and other capabilities described in the architecture documents are not necessarily implemented.

## Implemented features

- Rust 2024 workspace; the facade re-exports core and has no default features.
- Shared asynchronous Client with immutable configuration; requires a Tokio runtime with time enabled. The SDK does not create a runtime.
- Static credentials, explicit environment credential loading, temporary tokens, and custom asynchronous credential providers.
- Optional credential files, CVM instance roles, STS AssumeRole, TKE/OIDC, an explicitly selected default credential chain, shared caching, and on-demand refresh.
- TC3-HMAC-SHA256, POST JSON, fixed operation descriptors, and dynamic generic calls.
- `cos`: a separate COS XML REST client with common object/bucket operations, multipart uploads, and presigned URLs. See the [COS guide](crates/cos/README.md).
- `ses`: 36 typed SES 2020-10-02 APIs using core. See the [SES guide](crates/ses/README.md).
- `ssm`: 24 typed SSM 2019-09-23 APIs using core. See the [SSM guide](crates/ssm/README.md).
- Replaceable Transport, with an optional pooled reqwest/Rustls implementation.
- Per-product HTTPS root endpoint overrides and per-call region, language, header, and timeout options.
- Common JSON response envelopes, typed decoding, and preserved official error codes and RequestId.
- Bounded request encoding and actual response-byte reads; default credential and request debug output is redacted.

## Dependencies and usage

Use a local path dependency when working from source:

```toml
[dependencies]
tencentcloud-sdk = { path = "../tencentcloud-sdk", default-features = false, features = ["transport-reqwest"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The following example makes a real query and requires explicit application authorization. The repository's default tests do not execute it:

```rust,no_run
use tencentcloud_sdk::{ApiResponse, Client, CommonClient};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .credentials_from_env()
        .region("ap-shanghai")
        .build()?;
    let cvm = CommonClient::new(client, "cvm", "2017-03-12")?;
    let response: ApiResponse<Value> = cvm
        .call("DescribeInstances", &json!({"Limit": 1}))
        .await?;
    println!("RequestId: {:?}", response.metadata.request_id);
    Ok(())
}
```

`credentials_from_env()` reads `TENCENTCLOUD_SECRET_ID`, `TENCENTCLOUD_SECRET_KEY`, and optional `TENCENTCLOUD_TOKEN` only when building the client. It does not probe instance metadata, call STS, or reread the environment for each request. This static snapshot has no automatic expiration management. For role credentials and refresh, explicitly enable `credentials-roles` and select a provider. See the [credentials guide](docs/credentials.md).

You can also use `Operation::new(service, version, action)` with `Client::execute` and custom serde request/response types. This uses the same call pipeline as CommonClient; every request must encode as a JSON object.

Without `transport-reqwest`, inject a `Transport` through the builder; otherwise building returns a configuration error. Core does not implicitly enable network dependencies.

## Defaults and limitations

- Each call sends one request with no automatic retry. The default transport disables redirects and environment proxies and verifies TLS certificates.
- Custom transports must not redirect, automatically replay, or re-encode requests.
- Defaults: 10-second connection timeout, 60-second call budget, and 32 MiB limits for both request and response bodies.
- The budget includes credential acquisition and response-body reads. Synchronous encoding/decoding cannot be preempted; deadlines are checked at stage boundaries.
- Endpoints must be HTTPS root URLs, without arbitrary paths or query strings. HTTP is allowed only for explicitly selected numeric loopback endpoints in local tests.
- Tencent Cloud JSON errors are parsed even for non-200 HTTP responses. Missing Content-Type permits a JSON attempt; when present, it must be application/json, optionally with a charset.
- Service error messages may contain business data. They are available through explicit field access and omitted from default Display/Debug. Explicit access to error sources may still disclose sensitive information.
- Credential files and metadata are not probed by default. File and role credentials must be explicitly enabled and selected; the SDK does not persist secrets.
- API 3.0 calls support POST JSON only. COS uses a separate XML REST/binary protocol with HMAC-SHA1. Generic legacy signing, HTTP multipart/form-data, SSE, automatic pagination, concurrency limits, and disaster-recovery failover are not implemented.

## Local checks

```bash
cargo fmt --all --check
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
cargo tree -e normal --no-default-features
bash scripts/check-coverage.sh
```

Production-code line coverage with all features must be **at least 90%**; the current measured result is **95.72%**. See [test coverage](docs/coverage.md) for scope, tooling, and reporting commands.

Tests use fixed fake credentials and MockTransport. HTTP adapter tests use only ephemeral 127.0.0.1 ports and do not require real Tencent Cloud credentials. Expected signatures are checked against independent Python hashlib/hmac calculations.

The minimum supported Rust version, **1.85.0**, has been verified on macOS ARM. Offline performance benchmarks use the release profile and run explicitly, outside default tests and SDK runtime dependencies. See the [validation report](docs/validation.md) for the environment, results, and reproduction commands.

## Releases

Release Please creates or updates PRs for the shared workspace version and [changelog](CHANGELOG.md) on pushes to `main`. A separate publishing workflow tags the merged version PR commit as `vX.Y.Z`, creates a GitHub Release with that version's changelog notes, and publishes all five crates to crates.io in dependency order. The SDK is licensed under [Apache-2.0](LICENSE). See the [release guide](docs/releases.md) for credentials, safety checks, and recovery.

## Design documents

- [Credentials guide](docs/credentials.md)
- [Architecture](docs/architecture.md)
- [Modularity and opt-in dependencies](docs/modularity.md)
- [Performance and offline benchmarks](docs/performance.md)

## Third-party attribution

The SDK's SES/SSM protocol metadata references [Tencent Cloud Java SDK](https://github.com/TencentCloud/tencentcloud-sdk-java), version 3.2.40. Product-specific API versions are recorded in the product guides and schema snapshots.

Copyright (c) 2017-2025 Tencent. All Rights Reserved.

Upstream material is licensed under Apache-2.0; see `LICENSE`. Annotated protocol field names/types and operation identifiers were transformed into Rust serde models, typed action bindings, schema manifests, and test fixtures. Rust naming, optional-field handling, model layout, and redacted Debug implementations are specific to this repository. Java implementation logic, inheritance, generated accessors, and source comments are not used.
