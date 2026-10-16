use crate::{Error, Response, ServiceError};
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use http::HeaderMap;
use quick_xml::{Reader, events::Event};
use serde::{Deserialize, de::DeserializeOwned};
use tencentcloud_sdk_core::{SdkError, transport::TransportResponse};

pub(crate) async fn collect(
    mut response: TransportResponse,
    limit: usize,
) -> Result<Response<Bytes>, Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = response.body.next().await {
        let chunk = chunk?;
        if chunk.len() > limit.saturating_sub(body.len()) {
            return Err(SdkError::BodyTooLarge { limit }.into());
        }
        body.extend_from_slice(&chunk);
    }
    let response = Response {
        data: body.freeze(),
        request_id: header(&response.headers, "x-cos-request-id"),
        status: response.status,
        headers: response.headers,
    };
    if !response.status.is_success() {
        return Err(service_error(&response).unwrap_or_else(|| {
            protocol_error(
                &response,
                "COS returned a non-success response without a valid XML error",
            )
        }));
    }
    Ok(response)
}

pub(crate) fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}
fn protocol_error(response: &Response<Bytes>, message: &'static str) -> Error {
    SdkError::Protocol {
        message,
        status: response.status,
        request_id: response.request_id.clone(),
    }
    .into()
}

pub(crate) fn decode<T: DeserializeOwned>(
    response: Response<Bytes>,
    expected_root: &str,
) -> Result<Response<T>, Error> {
    let root =
        root(&response.data).map_err(|()| protocol_error(&response, "invalid COS XML document"))?;
    if root == "Error" {
        return Err(service_error(&response)
            .unwrap_or_else(|| protocol_error(&response, "invalid COS XML error")));
    }
    if root != expected_root {
        return Err(protocol_error(
            &response,
            "unexpected COS XML response root",
        ));
    }
    let data = quick_xml::de::from_reader(response.data.as_ref())
        .map_err(|_| protocol_error(&response, "invalid COS XML response"))?;
    Ok(Response {
        data,
        headers: response.headers,
        status: response.status,
        request_id: response.request_id,
    })
}
fn root(body: &[u8]) -> Result<String, ()> {
    let mut reader = Reader::from_reader(body);
    loop {
        match reader.read_event().map_err(|_| ())? {
            Event::Start(event) | Event::Empty(event) => {
                return String::from_utf8(event.local_name().as_ref().to_vec()).map_err(|_| ());
            }
            Event::Decl(_) | Event::Comment(_) | Event::PI(_) => {}
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(()),
        }
    }
}
fn service_error(response: &Response<Bytes>) -> Option<Error> {
    if root(&response.data).ok()?.as_str() != "Error" {
        return None;
    }
    let wire: WireError = quick_xml::de::from_reader(response.data.as_ref()).ok()?;
    Some(Error::Service(ServiceError {
        code: wire.code,
        message: wire.message,
        request_id: wire
            .request_id
            .filter(|id| !id.is_empty())
            .or_else(|| response.request_id.clone()),
        status: response.status,
    }))
}
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WireError {
    code: String,
    message: String,
    request_id: Option<String>,
}

pub(crate) fn configuration(message: &'static str) -> Error {
    SdkError::Configuration(message).into()
}
pub(crate) fn ensure_success_empty(response: Response<Bytes>) -> Result<Response<()>, Error> {
    if !response.data.is_empty() {
        return Err(protocol_error(
            &response,
            "unexpected body for a bodyless COS operation",
        ));
    }
    Ok(Response {
        data: (),
        headers: response.headers,
        status: response.status,
        request_id: response.request_id,
    })
}
