use super::*;

#[test]
fn error_formatting_and_sources_preserve_categories_without_leaking_details() {
    let service = ServiceError {
        code: "AuthFailure.Test".into(),
        message: "private-service-detail".into(),
        request_id: Some("test-request".into()),
        status: StatusCode::FORBIDDEN,
    };
    assert!(format!("{service:?}").contains("AuthFailure.Test"));
    assert!(!format!("{service:?}").contains("private-service-detail"));
    let serialization = serde_json::from_str::<serde_json::Value>("!").unwrap_err();
    let errors = [
        (
            SdkError::Configuration("test configuration"),
            "client configuration error",
            false,
        ),
        (
            SdkError::Credentials("test credentials"),
            "credentials error",
            false,
        ),
        (SdkError::Signing("test signing"), "signing error", false),
        (
            SdkError::Serialization(serialization),
            "failed to encode request JSON",
            true,
        ),
        (
            SdkError::Transport(Box::new(std::io::Error::other("private-transport-detail"))),
            "HTTP transport failed",
            true,
        ),
        (SdkError::Timeout, "call timed out", false),
        (
            SdkError::BodyTooLarge { limit: 17 },
            "exceeds 17 bytes",
            false,
        ),
        (
            SdkError::Protocol {
                message: "test protocol",
                status: StatusCode::BAD_GATEWAY,
                request_id: Some("protocol-id".into()),
            },
            "response protocol error",
            false,
        ),
        (SdkError::Service(service), "AuthFailure.Test", true),
    ];
    for (error, expected, has_source) in errors {
        let display = error.to_string();
        assert!(display.contains(expected), "{display}");
        assert_eq!(display, format!("{error:?}"));
        assert!(!display.contains("private-service-detail"));
        assert!(!display.contains("private-transport-detail"));
        assert_eq!(error.source().is_some(), has_source);
    }
}
