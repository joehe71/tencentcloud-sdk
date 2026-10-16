use crate::{
    Credentials,
    cos_signing::{authorize, encode_component, presign},
    transport::TransportRequest,
};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method};
use std::time::Duration;

fn request() -> TransportRequest {
    let mut headers = HeaderMap::new();
    headers.insert("range", HeaderValue::from_static("bytes=0-3"));
    TransportRequest { method: Method::GET, uri: "https://bucket-123.cos.ap-shanghai.myqcloud.com/a%20b/%E4%B8%AD%2B%25?prefix=A%2FB&marker=x%2By&uploads=".parse().unwrap(), headers, body: Bytes::new() }
}
#[test]
fn signature_matches_independent_python_vector_and_binds_token_headers_and_path() {
    let credentials = Credentials::new("test-id", "test-key")
        .unwrap()
        .with_token("test-token")
        .unwrap();
    let mut req = request();
    authorize(
        &credentials,
        &mut req,
        1_700_000_000,
        Duration::from_secs(900),
    )
    .unwrap();
    let auth = req.headers["authorization"].to_str().unwrap();
    assert!(auth.ends_with("q-signature=304cf22e4a294d26688eed40a99d71327aede87e"));
    assert!(auth.contains("q-header-list=host;range;x-cos-security-token"));
    assert!(auth.contains("q-url-param-list=marker;prefix;uploads"));
    assert!(req.headers["authorization"].is_sensitive());
    assert!(req.headers["x-cos-security-token"].is_sensitive());
    assert!(!format!("{req:?}").contains("test-token"));
    assert_eq!(encode_component("a +/%中~"), "a%20%2B%2F%25%E4%B8%AD~");
}
#[test]
fn presign_query_encodes_tokens_and_keeps_input_unchanged() {
    let credentials = Credentials::new("test-id", "test-key")
        .unwrap()
        .with_token("token+/=")
        .unwrap();
    let req = request();
    let url = presign(&credentials, &req, 1_700_000_000, Duration::from_secs(60)).unwrap();
    assert!(url.contains("x-cos-security-token=token%2B%2F%3D"));
    assert!(url.contains("q-sign-time=1700000000%3B1700000060"));
    assert!(url.contains("q-url-param-list=marker%3Bprefix%3Buploads%3Bx-cos-security-token"));
    assert!(!req.headers.contains_key("host"));
    assert!(req.uri.query().unwrap().ends_with("uploads="));
    let plain = Credentials::new("test-id", "test-key").unwrap();
    let mut no_query = request();
    no_query.uri = "https://example.com/object".parse().unwrap();
    let url = presign(&plain, &no_query, 0, Duration::from_secs(1)).unwrap();
    assert!(url.contains("/object?q-sign-algorithm=sha1"));
}
#[test]
fn rejects_invalid_signing_inputs_and_handles_repeated_query_values() {
    let credentials = Credentials::new("test-id", "test-key").unwrap();
    for (time, duration) in [
        (-1, Duration::from_secs(1)),
        (0, Duration::ZERO),
        (0, Duration::from_secs(604_801)),
        (i64::MAX, Duration::from_secs(1)),
    ] {
        assert!(authorize(&credentials, &mut request(), time, duration).is_err());
    }
    for uri in [
        "/relative",
        "https://example.com/%FF",
        "https://example.com/?q-ak=evil",
        "https://example.com/?key=%FF",
    ] {
        let mut req = request();
        req.uri = uri.parse().unwrap();
        assert!(authorize(&credentials, &mut req, 0, Duration::from_secs(60)).is_err());
    }
    let mut req = request();
    req.headers
        .insert("authorization", HeaderValue::from_static("already-signed"));
    assert!(authorize(&credentials, &mut req, 0, Duration::from_secs(60)).is_err());
    let mut req = request();
    req.uri = "https://example.com/?KEY=z&KEY=a&empty&&space=x+y"
        .parse()
        .unwrap();
    authorize(&credentials, &mut req, 0, Duration::from_secs(60)).unwrap();
    assert!(
        req.headers["authorization"]
            .to_str()
            .unwrap()
            .contains("q-url-param-list=empty;key;space")
    );
    let mut req = request();
    req.headers
        .insert("x-cos-meta-test", HeaderValue::from_bytes(&[0xff]).unwrap());
    assert!(authorize(&credentials, &mut req, 0, Duration::from_secs(60)).is_err());
}
