# tencentcloud-sdk-foundation

Shared asynchronous client, TC3-HMAC-SHA256 signing, credentials, transport abstraction, and Tencent Cloud API 3.0 error handling for the modular Tencent Cloud Rust SDK.

The package `tencentcloud-sdk-foundation` uses the matching Rust library name `tencentcloud_sdk_foundation`. Import it as `tencentcloud_sdk_foundation`; no alias for the old Rust library name is provided. The unrelated `tencentcloud-sdk-core` package on crates.io is not part of this repository.

Optional features: `credentials-file`, `credentials-roles`, `transport-reqwest`, and `cos-signing`. Default features are empty. Product clients are distributed separately; the core does not depend on them.

See the [repository](https://github.com/joehe71/tencentcloud-sdk) for API guides and examples.

Licensed under Apache-2.0. See `LICENSE`.

Third-party attribution is provided in `NOTICE`, included in this package.
