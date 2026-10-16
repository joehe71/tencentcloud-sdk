use crate::{Credentials, SdkError, signing::hex, transport::TransportRequest};
use hmac::{Hmac, KeyInit, Mac};
use http::{HeaderValue, Uri};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use sha1::{Digest, Sha1};
use std::{collections::BTreeMap, time::Duration};

const ESCAPE: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');
type Values = BTreeMap<String, Vec<String>>;

/// Encodes one UTF-8 URL component using COS's RFC 3986 encoding rules.
#[must_use]
pub fn encode_component(value: &str) -> String {
    utf8_percent_encode(value, ESCAPE).to_string()
}

/// Signs a request and attaches its snapshot's temporary token without exposing secret material.
///
/// All supplied headers are signed; transports must not modify them. Paths are decoded once
/// for the canonical string, while the wire URI remains encoded. Bodies are not part of COS's
/// signature; use HTTPS and Content-MD5 where integrity checks are required.
///
/// # Errors
/// Returns an error for invalid URIs, header values, timestamps, or validity outside [1s, 7d].
pub fn authorize(
    credentials: &Credentials,
    request: &mut TransportRequest,
    start: i64,
    validity: Duration,
) -> Result<(), SdkError> {
    prepare_host(request)?;
    if let Some(token) = credentials.token() {
        request
            .headers
            .insert("x-cos-security-token", token.clone());
    }
    let authorization = signature(credentials, request, start, validity)?;
    request.headers.insert("authorization", authorization);
    Ok(())
}

/// Creates a presigned URI, placing temporary tokens in the signed query, not a required header.
///
/// The returned string is a bearer capability and must never be logged. Supplied headers
/// other than Host remain mandatory for the URL's eventual consumer. Credential expiry may
/// invalidate the URL before its requested lifetime ends.
///
/// # Errors
/// Returns the same validation errors as [`authorize`].
pub fn presign(
    credentials: &Credentials,
    request: &TransportRequest,
    start: i64,
    validity: Duration,
) -> Result<String, SdkError> {
    let mut request = request.clone();
    prepare_host(&mut request)?;
    if let Some(token) = credentials.token() {
        append_query(
            &mut request.uri,
            &format!(
                "x-cos-security-token={}",
                encode_component(
                    token
                        .to_str()
                        .map_err(|_| SdkError::Credentials("COS token is not ASCII"))?
                )
            ),
        )?;
    }
    let authorization = signature(credentials, &request, start, validity)?;
    let query = authorization
        .to_str()
        .map_err(|_| SdkError::Signing("invalid COS authorization"))?
        .split('&')
        .map(|item| {
            let (key, value) = item.split_once('=').unwrap_or((item, ""));
            format!("{key}={}", encode_component(value))
        })
        .collect::<Vec<_>>()
        .join("&");
    append_query(&mut request.uri, &query)?;
    Ok(request.uri.to_string())
}

fn prepare_host(request: &mut TransportRequest) -> Result<(), SdkError> {
    if request.headers.contains_key("authorization") {
        return Err(SdkError::Configuration(
            "COS requires an unsigned input request",
        ));
    }
    let host = request
        .uri
        .authority()
        .ok_or(SdkError::Configuration("COS URI has no authority"))?
        .as_str();
    let value =
        HeaderValue::from_str(host).map_err(|_| SdkError::Configuration("invalid COS host"))?;
    request.headers.insert("host", value);
    Ok(())
}
fn append_query(uri: &mut Uri, query: &str) -> Result<(), SdkError> {
    *uri = format!(
        "{}{separator}{query}",
        uri,
        separator = if uri.query().is_some() { '&' } else { '?' }
    )
    .parse()
    .map_err(|_| SdkError::Configuration("invalid COS query URI"))?;
    Ok(())
}
fn decode(value: &str) -> Result<String, SdkError> {
    percent_decode_str(value)
        .decode_utf8()
        .map(std::borrow::Cow::into_owned)
        .map_err(|_| SdkError::Configuration("COS URI component is not UTF-8"))
}
fn signature(
    credentials: &Credentials,
    request: &TransportRequest,
    start: i64,
    validity: Duration,
) -> Result<HeaderValue, SdkError> {
    if start < 0 || validity.as_secs() == 0 || validity > Duration::from_secs(604_800) {
        return Err(SdkError::Configuration(
            "COS signature validity must be in [1s, 7d] with a nonnegative timestamp",
        ));
    }
    let end = start
        .checked_add(
            i64::try_from(validity.as_secs())
                .map_err(|_| SdkError::Signing("COS signature duration overflow"))?,
        )
        .ok_or(SdkError::Signing("COS signature expiration overflow"))?;
    let key_time = format!("{start};{end}");
    let mut query = Values::new();
    if let Some(raw) = request.uri.query() {
        for pair in raw.split('&').filter(|item| !item.is_empty()) {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            let key = decode(&key.replace('+', " "))?;
            if key.to_ascii_lowercase().starts_with("q-") {
                return Err(SdkError::Configuration(
                    "reserved COS authorization query parameter",
                ));
            }
            add(&mut query, &key, decode(&value.replace('+', " "))?);
        }
    }
    let mut headers = Values::new();
    for (name, value) in &request.headers {
        add(
            &mut headers,
            name.as_str(),
            value
                .to_str()
                .map_err(|_| SdkError::Configuration("COS signing headers must be ASCII"))?
                .to_owned(),
        );
    }
    let canonical = format!(
        "{}\n{}\n{}\n{}\n",
        request.method.as_str().to_ascii_lowercase(),
        decode(request.uri.path())?,
        canonical_values(&query),
        canonical_values(&headers)
    );
    let sign_key = hmac(credentials.secret_key().as_bytes(), key_time.as_bytes())?;
    let to_sign = format!(
        "sha1\n{key_time}\n{}\n",
        hex(&Sha1::digest(canonical.as_bytes()))
    );
    let digest = hmac(sign_key.as_bytes(), to_sign.as_bytes())?;
    let mut value = HeaderValue::from_str(&format!("q-sign-algorithm=sha1&q-ak={}&q-sign-time={key_time}&q-key-time={key_time}&q-header-list={}&q-url-param-list={}&q-signature={digest}", credentials.secret_id(), headers.keys().cloned().collect::<Vec<_>>().join(";"), query.keys().cloned().collect::<Vec<_>>().join(";")))
        .map_err(|_| SdkError::Signing("invalid COS authorization header"))?;
    value.set_sensitive(true);
    Ok(value)
}
fn add(map: &mut Values, key: &str, value: String) {
    map.entry(encode_component(key).to_ascii_lowercase())
        .or_default()
        .push(value);
}
fn canonical_values(map: &Values) -> String {
    map.iter()
        .flat_map(|(key, values)| {
            let mut values = values.clone();
            values.sort();
            values
                .into_iter()
                .map(move |value| format!("{key}={}", encode_component(&value)))
        })
        .collect::<Vec<_>>()
        .join("&")
}
fn hmac(key: &[u8], value: &[u8]) -> Result<String, SdkError> {
    let mut mac = Hmac::<Sha1>::new_from_slice(key)
        .map_err(|_| SdkError::Signing("failed to initialize COS HMAC"))?;
    mac.update(value);
    Ok(hex(&mac.finalize().into_bytes()))
}

#[cfg(test)]
#[path = "tests/cos_signing.rs"]
mod tests;
