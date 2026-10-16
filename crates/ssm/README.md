# Tencent Cloud Secrets Manager

Typed asynchronous API 3.0 calls for SSM **2019-09-23**. Enable the facade's optional `ssm` feature to use `tencentcloud_sdk::ssm::v20190923`, or depend directly on `tencentcloud-sdk-ssm`. Packages are not published yet.

## Use

The product wraps a preconfigured core client and adds only serde. It does not depend on COS or SES. Enable `transport-reqwest` in the facade for the core HTTP pool, or inject a transport. File/role credentials are separate opt-in features; the existing TKE OIDC, CVM and `AssumeRole` providers work without SSM-specific credential logic. The SDK never retrieves or installs a stored secret as the caller's cloud identity automatically.

```rust,no_run
use tencentcloud_sdk_core::{Client as CoreClient, SdkError};
use tencentcloud_sdk_ssm::v20190923::{Client, models::GetSecretValueRequest};

async fn read_secret(core: CoreClient) -> Result<(), SdkError> {
    let client = Client::new(core);
    let reply = client.get_secret_value(&GetSecretValueRequest {
        secret_name: Some("example-secret".into()),
        version_id: Some("SSM_Current".into()),
        ..Default::default()
    }).await?;
    // Explicit access returns application data. Never log or print this value.
    let _plaintext = reply.data.secret_string;
    let _request_id = reply.metadata.request_id;
    Ok(())
}
```

Configure a supported region explicitly on core or through `execute_with_options`. The core endpoint override key is `ssm`; the default endpoint is `https://ssm.tencentcloudapi.com/`, the TC3 service is `ssm` and the signed API version is `2019-09-23`. Construction does no I/O. Clones share the existing core, credential-refresh cache and connection pool.

## Operations

All **24 actions in the referenced Java snapshot** have typed methods:

- Secrets: `create_secret`, `get_secret_value`, `describe_secret`, `list_secrets`, `update_secret`, `update_description`, `enable_secret`, `disable_secret`.
- Versions: `put_secret_value`, `list_secret_version_ids`, `delete_secret_version`.
- Deletion/recovery: `delete_secret`, `restore_secret`.
- Product rotation: `rotate_product_secret`, `update_rotation_status`, `describe_rotation_detail`, `describe_rotation_history`.
- SSH keys: `create_ssh_key_pair_secret`, `get_ssh_key_pair_value`.
- Product/service discovery: `create_product_secret`, `describe_supported_products`, `get_regions`, `get_service_status`.
- Asynchronous request status: `describe_async_request_info`.

The sealed `Request` trait pairs every input with the original action and correct response type. Methods execute once; `execute` and `execute_with_options` also accept any supported typed request. Polling, rotation scheduling and pagination are application decisions, not hidden SDK tasks.

## Data handling and safety

- The 54 models use explicit official JSON renames, including `SSHKeyID`, `ProjectID`, `CleanSSHKey` and `KmsHsmClusterId`. Rust identifiers normalize acronyms without changing their wire spelling. Large integers remain `i64`, not floating-point values.
- Fields use `Option<T>`: unset fields are omitted, response null/missing fields become None, and unknown fields are ignored. Required-field rules, string/binary exclusivity, deletion modes, recovery windows, version constraints and product-specific permissions are validated by the service, not by a claimed compile-time validator. `Default` alone is not necessarily a valid request.
- `SecretString` is plaintext. `SecretBinary`, SSH public keys and private keys retain the service's Base64 strings; no automatic decoding or re-encoding occurs. Explicit JSON serialization also contains secrets. Applications must not log, persist or send these values to diagnostics inadvertently.
- Optional `EncryptionPublicKey` and `EncryptionAlgorithm` fields are wire-level pass-through parameters. The SDK neither selects an algorithm nor performs application-side encryption/decryption; applications using those options must interpret the service's response according to its protocol.
- Every model and product-client Debug implementation hides field values. Core transport/error formatting remains redacted; official error code, message and `RequestId` remain explicitly accessible. Accessing error sources or model fields is an intentional disclosure boundary.
- Values are ordinary owned Rust strings. This crate does **not** promise encrypted process memory, automatic zeroization, restricted cloning, secret-value caching, on-disk storage, logging hooks or filesystem installation of SSH keys. Applications needing those properties must supply their own handling.
- Every read reaches the core transport; there is no hidden secret-value cache. Reads and writes inherit the existing body bounds and call budget. Set `max_request_body`, `max_response_body` and `timeout` on core according to the application and service limits.
- Writes, version deletion, credential deletion, key creation and product rotation are sent **once**. The SDK never adds a deletion mode or recovery window, forces deletion, deletes a related SSH key, rotates a credential after a read, replays a failed mutation, follows up with a second business call or polls automatically. Server defaults still apply when fields are omitted; applications must choose explicit deletion parameters carefully.
- List/history methods return a single page. Pass the appropriate Offset/Limit and inspect totals explicitly. Service status and asynchronous task responses are returned as-is, not interpreted as readiness guarantees.

## Offline verification and protocol reference

Tests cover all 24 typed action routes and every field of all 54 models, including null handling and Debug redaction. Hand-written scenarios verify exact text/binary creation, tags and version selectors, secret reads without cache, Base64 SSH keys and large project IDs, explicit deletion parameters, original errors without replay, pagination overrides, bounded bodies, zero timeout and malformed typed responses. All inputs are fake and transports are local mocks; no secret, key or cloud resource is created, retrieved, deleted or rotated in a real account.

Protocol metadata references Tencent Cloud Java SDK 3.2.40, `ssm/v20190923`. The checked-in schema records the snapshot, not a claim to implement newer actions. Cargo does not invoke code generation or need the Java checkout. Maintain the Rust models, typed bindings, schema snapshot, and fixtures together when updating the supported protocol.

```bash
cargo fmt --all
cargo test -p tencentcloud-sdk-ssm
bash scripts/check-coverage.sh
```

The reference contributes annotated wire metadata, not Java behavior or comments. Model fixtures supplement independent hand-written fixtures. Upstream attribution and Apache-2.0 terms are in `THIRD_PARTY_NOTICES.md` and `LICENSE`. See [official SSM documentation](https://cloud.tencent.com/document/product/1140).
