//! Exercise process-environment entry points in isolated children, never the caller's environment.
use std::{process::Command, sync::Arc};
use tencentcloud_sdk_core::{
    BoxFuture, Client, SdkError,
    credentials::{EnvironmentCredentialsProvider, ProvideCredentials},
    transport::{Transport, TransportRequest, TransportResponse},
};

struct NoNetwork;
impl Transport for NoNetwork {
    fn send(&self, _: TransportRequest) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        Box::pin(async { panic!("environment tests must never send network requests") })
    }
}

#[test]
fn public_environment_entry_points_are_isolated_from_ambient_credentials() {
    for mode in ["complete", "missing"] {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "environment_child", "--ignored", "--nocapture"])
            .env_clear()
            .env("SDK_TEST_ENV_CASE", mode);
        // Preserve instrumentation only; never inherit ambient credentials, HOME, or proxy settings.
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        if mode == "complete" {
            for (name, value) in [
                ("TENCENTCLOUD_SECRET_ID", "test-id"),
                ("TENCENTCLOUD_SECRET_KEY", "test-key"),
                ("TENCENTCLOUD_TOKEN", "test-token"),
                ("TKE_REGION", "ap-shanghai"),
                ("TKE_PROVIDER_ID", "test-provider"),
                ("TKE_ROLE_ARN", "qcs::cam::uin/123:roleName/test"),
                ("TKE_WEB_IDENTITY_TOKEN_FILE", "unused-test-token-file"),
            ] {
                command.env(name, value);
            }
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "environment child failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[tokio::test]
#[ignore = "invoked only by the isolated child-process test"]
async fn environment_child() {
    let complete = std::env::var("SDK_TEST_ENV_CASE").unwrap() == "complete";
    let environment = EnvironmentCredentialsProvider::load();
    let client = Client::builder()
        .credentials_from_env()
        .shared_transport(Arc::new(NoNetwork))
        .build();
    assert_eq!(environment.is_ok(), complete);
    assert_eq!(client.is_ok(), complete);
    if let Ok(provider) = environment {
        let first = provider.provide_credentials().await.unwrap();
        assert!(Arc::ptr_eq(
            &first,
            &provider.provide_credentials().await.unwrap()
        ));
    }
    #[cfg(feature = "credentials-roles")]
    {
        use tencentcloud_sdk_core::credentials::{
            DefaultCredentialsProvider, OidcRoleCredentialsProvider, RoleProviderOptions,
        };
        let options = RoleProviderOptions::new(Arc::new(NoNetwork));
        let oidc = OidcRoleCredentialsProvider::from_tke_env(options.clone());
        assert_eq!(oidc.is_ok(), complete);
        if complete {
            let chain = DefaultCredentialsProvider::new(options).unwrap();
            assert!(format!("{chain:?}").contains("DefaultCredentialsProvider"));
            let first = chain.provide_credentials().await.unwrap();
            assert!(Arc::ptr_eq(
                &first,
                &chain.clone().provide_credentials().await.unwrap()
            ));
        }
    }
}
