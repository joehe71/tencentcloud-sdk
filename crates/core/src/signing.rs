use hmac::{Hmac, KeyInit, Mac};
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
        return Err(SdkError::Signing(
            "signing time must not be before the Unix epoch",
        ));
    }
    let date = OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|_| SdkError::Signing("signing time is out of range"))?
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
    )).map_err(|_| SdkError::Signing("failed to construct Authorization"))?;
    header.set_sensitive(true);
    Ok(header)
}

fn hmac256(key: &[u8], data: &[u8]) -> Result<[u8; 32], SdkError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
        .map_err(|_| SdkError::Signing("failed to initialize HMAC"))?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().into())
}

fn sha256_hex(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
    output
}

#[cfg(test)]
#[path = "tests/signing.rs"]
mod tests;
