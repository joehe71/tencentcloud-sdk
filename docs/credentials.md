# Credentials

Credentials are immutable snapshots. A signature and its session token always come from the same snapshot. Providers implement `ProvideCredentials`; cloning a role provider shares its cache and refresh gate.

## Features and explicit selection

| Feature | Capability |
| --- | --- |
| No credential feature | Static credentials, explicit environment snapshots, custom providers |
| `credentials-file` | Explicit INI profile loading |
| `credentials-roles` | Profile loading, CVM metadata, signed STS AssumeRole, OIDC, explicit default chain |
| `transport-reqwest` | Optional HTTP implementation, independent of credential features |

These features are available on both the facade and core crates. Defaults remain empty. Enabling a feature does not select a provider or probe a credential source. Role providers require an injected transport, allowing applications to share one connection pool between credentials and business calls. No additional third-party crate was added for credential management; optional Tokio capabilities are enabled only where needed.

## TKE Pod OIDC

Enable `credentials-roles` and `transport-reqwest` in the dependency's feature list. In a TKE workload configured for CAM OIDC access, use the explicit OIDC provider rather than the default chain to prevent older environment or profile keys from taking precedence.

```rust,no_run
use std::sync::Arc;
use tencentcloud_sdk::{Client, credentials::{OidcRoleCredentialsProvider, RoleProviderOptions},
    transport::ReqwestTransport};

fn build_client() -> Result<Client, tencentcloud_sdk::SdkError> {
    let transport = Arc::new(ReqwestTransport::new(std::time::Duration::from_secs(10))?);
    let provider = OidcRoleCredentialsProvider::from_tke_env(
        RoleProviderOptions::new(transport.clone()),
    )?;
    Client::builder()
        .credentials_provider(provider)
        .shared_transport(transport)
        .region("ap-shanghai")
        .build()
}
```

The example only constructs a client. It does not read a token file or send a request. Business calls obtain credentials lazily. The provider reads these configuration variables at construction:

- `TKE_REGION`
- `TKE_PROVIDER_ID`
- `TKE_ROLE_ARN`
- `TKE_WEB_IDENTITY_TOKEN_FILE`

On refresh, it reopens the projected token file, trims surrounding whitespace, and sends STS `AssumeRoleWithWebIdentity` using `Authorization: SKIP`, version `2018-08-13`. STS returns the temporary SecretId, SecretKey, Token and expiration; business requests are then signed with TC3 and the temporary Token. No permanent AK/SK is required.

The TKE cluster must enable OIDC resource access, the Pod must use the intended ServiceAccount, and CAM trust and permission policies must permit that identity and the business operations. Kubernetes RBAC is not CAM authorization. Connectivity to STS and the relevant business endpoints is required.

For non-TKE OIDC integrations, construct `OidcRoleConfig::new(region, provider_id, role_arn, token)` and pass it to `OidcRoleCredentialsProvider::new`. `WebIdentityToken::File` supports projected-file rotation; `Inline` is caller-managed and cannot rotate itself. Session names and requested durations can be overridden through configuration. The default requested duration is 7,200 seconds; server-side role limits remain authoritative.

## Other providers

- `StaticCredentialsProvider`: a caller-supplied immutable triple; no expiration tracking.
- `EnvironmentCredentialsProvider`: explicitly snapshots `TENCENTCLOUD_SECRET_ID`, `TENCENTCLOUD_SECRET_KEY`, and optional `TENCENTCLOUD_TOKEN`. The existing `ClientBuilder::credentials_from_env()` behavior is unchanged.
- `ProfileCredentialsProvider::load(path).await`: explicitly loads the `[default]` section containing `secret_id`, `secret_key`, and optional `token`. Files are bounded to 64 KiB and must be UTF-8. The parser supports blank lines, full-line `#`/`;` comments, and unquoted `key=value` entries, splitting at the first `=`. Unknown keys and non-default sections are ignored; duplicate known entries and partial credentials are rejected. It is not a general-purpose INI parser. Profiles are snapshots, not watched or automatically refreshed.
- `ProfileCredentialsProvider::load_default().await`: searches `/etc/tencentcloud/credentials`, then `$HOME/.tencentcloud/credentials`; on Windows, `$USERPROFILE/.tencentcloud/credentials` is tried first. The first existing file must be valid; malformed or unreadable files do not silently fall through.
- `CvmRoleCredentialsProvider::new(role_name, options)`: reads credentials from `http://metadata.tencentyun.com/latest/meta-data/cam/security-credentials/`. `None` discovers the role name before fetching the credential document. A specified role skips discovery. Metadata is a trusted instance-local HTTP service; never expose it as a general-purpose user-controlled URL.
- `AssumeRoleCredentialsProvider::new(source, config, options)`: signs STS `AssumeRole` using the supplied `Arc<dyn ProvideCredentials>`. `AssumeRoleConfig` sets region, ARN, session name and duration. Temporary source tokens are preserved, and the source provider is consulted on each refresh. This does not automatically turn ordinary environment variables into an AssumeRole configuration.

## Explicit default chain

`DefaultCredentialsProvider::new(options)` selects a provider lazily on first use:

1. Environment credentials, including an optional temporary Token.
2. The first existing default profile file.
3. TKE OIDC, if any of the four TKE configuration variables is present.
4. CVM instance-role metadata.

Partial or invalid configuration fails closed. TKE authentication or token-file failures never switch to the node role. After selection, the identity source is pinned for that provider's lifetime; environment and profile snapshots are not reselected on every request. Reconstruct the provider to deliberately change configuration.

This differs intentionally from Java's environment → profile → CVM → OIDC order: a configured Pod identity takes priority over the node role. Static environment and profile credentials still take precedence, so explicit `OidcRoleCredentialsProvider` is preferable for identity-isolated Pods.

## Refresh, timeouts, and failure behavior

- Refresh is lazy, performed by the first caller crossing the refresh threshold, not by a background task. There is no owned runtime or detached refresh loop.
- Default refresh window: 600 seconds. It is capped at half the remaining lifetime observed when accepting a credential, avoiding continuous refresh for short-lived credentials. CVM uses the same configurable Rust policy, rather than Java's fixed 300-second window.
- Unexpired snapshots before the threshold are shared through `Arc`. At the threshold, concurrent callers wait behind one asynchronous refresh gate and recheck the snapshot after acquiring it. Snapshot locks are never held across network or file awaits.
- Failed completed refreshes cause a five-second monotonic backoff. The initiating caller receives the original error, including STS error code and RequestId; callers during backoff receive a credential-backoff error. There is no automatic retry or identity fallback, and stale snapshots are not returned after a refresh failure.
- Each role-provider call has a default ten-second budget, including gate waits, token-file reads, source credentials, HTTP send, and response reads. The application's overall request budget still applies. Cancellation or provider timeout releases the gate without publishing partial state; a later caller can refresh again. Cancellation and timeout do not install the completed-failure backoff.
- Expired, partial, empty-token, or malformed credential responses are rejected. Token files are bounded to 64 KiB; credential HTTP request/response bodies are bounded to 1 MiB. The provider does not persist credentials or token files.
- Provider `Debug`, credentials formatting, request formatting and default error formatting redact sensitive values. Explicit access to service messages or transport error sources still requires care.
- Default transport disables redirects, automatic replay and environment proxies. Custom transports must preserve those guarantees. Root endpoint overrides are trusted application configuration; plaintext overrides are limited to explicit numeric loopback endpoints for offline tests.

## Offline verification

Tests use fake credentials, injectable clocks, in-memory transports, and files under the repository's ignored `target/` directory. They cover OIDC protocol fields, issued-triple signing, temporary source tokens, token rotation, shared snapshots, concurrent refresh, metadata discovery, injection rejection, cancellation, deadlines, invalid responses, bounds, failure backoff, default-chain ordering, and fail-closed behavior. They neither read ambient credential files nor modify process environment variables. No real STS, TKE, CVM, or CAM integration is claimed.
