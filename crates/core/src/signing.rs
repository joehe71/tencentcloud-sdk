use hmac::{Hmac, Mac};
use http::HeaderValue;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::{Credentials, SdkError};

pub(crate) const CONTENT_TYPE: &str = "application/json; charset=utf-8";
const SIGNED_HEADERS: &str = "content-type;host;x-tc-action";

pub(crate) fn authorization(
    credentials: &Credentials,
    service: &str,
    host: &str,
    action: &str,
    body: &[u8],
    timestamp: i64,
) -> Result<HeaderValue, SdkError> {
    if timestamp < 0 {
        return Err(SdkError::Signing("签名时间不能早于 Unix epoch"));
    }
    let date = OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|_| SdkError::Signing("签名时间超出有效范围"))?
        .date()
        .to_string();
    let canonical = format!(
        "POST\n/\n\ncontent-type:{CONTENT_TYPE}\nhost:{host}\nx-tc-action:{}\n\n{SIGNED_HEADERS}\n{}",
        action.to_ascii_lowercase(),
        sha256_hex(body),
    );
    let scope = format!("{date}/{service}/tc3_request");
    let string_to_sign = format!(
        "TC3-HMAC-SHA256\n{timestamp}\n{scope}\n{}",
        sha256_hex(canonical.as_bytes())
    );
    let secret_date = hmac256(
        format!("TC3{}", credentials.secret_key()).as_bytes(),
        date.as_bytes(),
    )?;
    let secret_service = hmac256(&secret_date, service.as_bytes())?;
    let secret_signing = hmac256(&secret_service, b"tc3_request")?;
    let signature = hex(&hmac256(&secret_signing, string_to_sign.as_bytes())?);
    let mut header = HeaderValue::from_str(&format!(
        "TC3-HMAC-SHA256 Credential={}/{scope}, SignedHeaders={SIGNED_HEADERS}, Signature={signature}",
        credentials.secret_id(),
    )).map_err(|_| SdkError::Signing("无法构造 Authorization"))?;
    header.set_sensitive(true);
    Ok(header)
}

fn hmac256(key: &[u8], data: &[u8]) -> Result<[u8; 32], SdkError> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key).map_err(|_| SdkError::Signing("无法初始化 HMAC"))?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().into())
}

fn sha256_hex(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_vector_matches_independent_python_implementation() {
        // 按官方 TC3 步骤，以假凭证由 Python hashlib/hmac 独立计算预期值。
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
}
