use std::{
    future::poll_fn,
    io::{self, Write},
};

use bytes::Bytes;
use http::{
    HeaderMap, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::value::RawValue;

use crate::{ApiResponse, ResponseMetadata, SdkError, ServiceError, transport::TransportResponse};

pub(crate) fn encode<I: Serialize + ?Sized>(input: &I, limit: usize) -> Result<Bytes, SdkError> {
    let mut writer = LimitedWriter {
        bytes: Vec::new(),
        limit,
        exceeded: false,
    };
    if let Err(error) = serde_json::to_writer(&mut writer, input) {
        return Err(if writer.exceeded {
            SdkError::BodyTooLarge { limit }
        } else {
            SdkError::Serialization(error)
        });
    }
    if writer.bytes.first() != Some(&b'{') {
        return Err(SdkError::Configuration("API 请求必须编码为 JSON 对象"));
    }
    Ok(Bytes::from(writer.bytes))
}

struct LimitedWriter {
    bytes: Vec<u8>,
    limit: usize,
    exceeded: bool,
}

impl Write for LimitedWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.len() > self.limit.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(io::Error::other("请求体超限"));
        }
        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) struct RawResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

pub(crate) async fn collect(
    mut response: TransportResponse,
    limit: usize,
) -> Result<RawResponse, SdkError> {
    if response
        .headers
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .is_some_and(|length| length > limit as u64)
    {
        return Err(SdkError::BodyTooLarge { limit });
    }
    let mut first = None;
    let mut combined: Option<Vec<u8>> = None;
    while let Some(chunk) = poll_fn(|cx| response.body.as_mut().poll_next(cx)).await {
        let chunk = chunk?;
        let current_len = combined
            .as_ref()
            .map_or_else(|| first.as_ref().map_or(0, Bytes::len), Vec::len);
        if chunk.len() > limit.saturating_sub(current_len) {
            return Err(SdkError::BodyTooLarge { limit });
        }
        if chunk.is_empty() {
            continue;
        }
        if let Some(buffer) = &mut combined {
            buffer.extend_from_slice(&chunk);
        } else if let Some(initial) = first.take() {
            let mut buffer =
                Vec::with_capacity(initial.len().saturating_add(chunk.len()).min(limit));
            buffer.extend_from_slice(&initial);
            buffer.extend_from_slice(&chunk);
            combined = Some(buffer);
        } else {
            first = Some(chunk);
        }
    }
    let body = combined.map_or_else(|| first.unwrap_or_default(), Bytes::from);
    Ok(RawResponse {
        status: response.status,
        headers: response.headers,
        body,
    })
}

#[derive(Deserialize)]
struct Envelope<'a> {
    #[serde(rename = "Response", borrow)]
    response: &'a RawValue,
}

#[derive(Deserialize)]
struct Probe {
    #[serde(rename = "Error")]
    error: Option<WireError>,
    #[serde(rename = "RequestId")]
    request_id: Option<String>,
}

#[derive(Deserialize)]
struct WireError {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}

pub(crate) fn decode<O: DeserializeOwned>(
    response: &RawResponse,
) -> Result<ApiResponse<O>, SdkError> {
    let status = response.status;
    let mut request_id = response
        .headers
        .get("x-tc-requestid")
        .and_then(|value| value.to_str().ok())
        .filter(|id| !id.is_empty())
        .map(str::to_owned);
    let protocol_error = |message, id| SdkError::Protocol {
        message,
        status,
        request_id: id,
    };
    if let Some(content_type) = response.headers.get(CONTENT_TYPE) {
        let content_type = content_type
            .to_str()
            .unwrap_or_default()
            .split(';')
            .next()
            .unwrap_or_default()
            .trim();
        if !content_type.eq_ignore_ascii_case("application/json") {
            return Err(protocol_error("仅支持 JSON 响应", request_id));
        }
    }
    let envelope: Envelope<'_> = serde_json::from_slice(&response.body)
        .map_err(|_| protocol_error("缺少有效的 Response 外层", request_id.clone()))?;
    let probe: Probe = serde_json::from_str(envelope.response.get())
        .map_err(|_| protocol_error("Response 公共字段无效", request_id.clone()))?;
    if let Some(id) = probe.request_id.filter(|id| !id.is_empty()) {
        request_id = Some(id);
    }
    if let Some(error) = probe.error {
        return Err(SdkError::Service(ServiceError {
            code: error.code,
            message: error.message,
            request_id,
            status,
        }));
    }
    if status != StatusCode::OK {
        return Err(protocol_error(
            "HTTP 非 200 且未返回腾讯云错误对象",
            request_id,
        ));
    }
    let data = serde_json::from_str(envelope.response.get())
        .map_err(|_| protocol_error("业务响应不符合目标类型", request_id.clone()))?;
    Ok(ApiResponse {
        data,
        metadata: ResponseMetadata { request_id, status },
    })
}
