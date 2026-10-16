use super::*;

#[test]
fn validates_roots_and_keeps_ports() {
    assert_eq!(
        Endpoint::https("https://CVM.tencentcloudapi.com:8443")
            .unwrap()
            .uri(),
        "https://cvm.tencentcloudapi.com:8443/"
    );
    for url in [
        "http://example.com",
        "https://id:key@example.com",
        "https://example.com/api",
        "https://example.com/?a=1",
        "https://example.com/#secret",
        "not-a-url",
    ] {
        assert!(Endpoint::https(url).is_err(), "{url}");
    }
    assert!(Endpoint::loopback_http("http://127.0.0.1:8080/").is_ok());
    assert!(Endpoint::loopback_http("http://[::1]:8080/").is_ok());
    assert!(Endpoint::loopback_http("http://example.com/").is_err());
    assert!(Endpoint::loopback_http("http://localhost/").is_err());
}
