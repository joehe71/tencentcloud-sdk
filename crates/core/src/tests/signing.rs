use super::*;

#[test]
fn hex_preserves_leading_zeroes_and_uses_lowercase() {
    assert_eq!(hex(&[]), "");
    assert_eq!(hex(&[0x00, 0x01, 0x0f, 0x10, 0xab, 0xff]), "00010f10abff");
}

#[test]
fn fixed_vector_matches_independent_python_implementation() {
    // Compute the expected value independently with Python hashlib/hmac and fake credentials using the official TC3 steps.
    let credentials = Credentials::new("test-id", "test-key").unwrap();
    let header = authorization(
        &credentials,
        "cvm",
        "cvm.tencentcloudapi.com",
        "DescribeInstances",
        br#"{"Limit":1}"#,
        1_551_113_065,
    )
    .unwrap();
    assert_eq!(
        header,
        "TC3-HMAC-SHA256 Credential=test-id/2019-02-25/cvm/tc3_request, SignedHeaders=content-type;host;x-tc-action, Signature=6581ca689bc1f876411008936abf21533fa22ec2cc3ae364dad0685ba61c9623"
    );
    assert!(header.is_sensitive());
}

#[test]
fn signs_exact_bytes_and_utc_date() {
    let credentials = Credentials::new("test-id", "test-key").unwrap();
    let sign = |body: &[u8], time| {
        authorization(
            &credentials,
            "cvm",
            "cvm.tencentcloudapi.com",
            "DescribeInstances",
            body,
            time,
        )
        .unwrap()
    };
    assert_ne!(sign(b"{}", 86_399), sign(b"{ }", 86_399));
    assert!(
        sign(b"{}", 86_399)
            .to_str()
            .unwrap()
            .contains("/1970-01-01/")
    );
    assert!(
        sign(b"{}", 86_400)
            .to_str()
            .unwrap()
            .contains("/1970-01-02/")
    );
    assert!(
        authorization(
            &credentials,
            "cvm",
            "cvm.tencentcloudapi.com",
            "Action",
            b"{}",
            -1
        )
        .is_err()
    );
    assert!(
        authorization(
            &credentials,
            "cvm",
            "cvm.tencentcloudapi.com",
            "Action",
            b"{}",
            i64::MAX
        )
        .is_err()
    );
}
