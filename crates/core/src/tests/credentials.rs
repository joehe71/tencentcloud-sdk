use super::*;

#[tokio::test]
async fn environment_is_loaded_once_without_mutating_process_state() {
    let mut reads = 0;
    let provider = EnvironmentCredentialsProvider::from_lookup(|name| {
        reads += 1;
        Ok(Some(
            match name {
                "TENCENTCLOUD_SECRET_ID" => "test-id",
                "TENCENTCLOUD_SECRET_KEY" => "test-key",
                _ => "test-token",
            }
            .to_owned(),
        ))
    })
    .unwrap();
    let first = provider.provide_credentials().await.unwrap();
    let second = provider.provide_credentials().await.unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(reads, 3);
    assert_eq!(first.token().unwrap(), "test-token");
    assert!(first.token().unwrap().is_sensitive());
}

#[test]
fn rejects_missing_invalid_and_empty_credentials() {
    assert!(EnvironmentCredentialsProvider::from_lookup(|_| Ok(None)).is_err());
    assert!(Credentials::new("", "test-key").is_err());
    assert!(Credentials::new("id\n", "test-key").is_err());
    assert!(Credentials::new("test-id", "").is_err());
    assert!(
        Credentials::new("test-id", "test-key")
            .unwrap()
            .with_token("")
            .is_err()
    );
    assert!(
        Credentials::new("test-id", "test-key")
            .unwrap()
            .with_token("x\ny")
            .is_err()
    );
}

#[test]
fn debug_does_not_expose_any_secret() {
    let credentials = Credentials::new("private-id", "private-key")
        .unwrap()
        .with_token("private-token")
        .unwrap();
    let debug = format!("{credentials:?}");
    for secret in ["private-id", "private-key", "private-token"] {
        assert!(!debug.contains(secret));
    }
}
