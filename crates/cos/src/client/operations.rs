use super::{Client, Request, Response, header, key_path, validate_bucket};
use crate::{
    Error,
    models::{
        CompleteMultipartResult, CompletedPart, CopyObjectResult, ListBucketsResult,
        ListObjectsOptions, ListObjectsResult, ListPartsResult, MultipartUpload,
    },
    protocol::configuration,
};
use bytes::Bytes;
use http::Method;
use serde::Serialize;
use std::ops::RangeInclusive;

impl Client {
    /// Uploads one bounded object with a Content-MD5 checksum. No automatic replay occurs.
    ///
    /// # Errors
    /// Returns input, credential, checksum-header, transport, deadline or COS service errors.
    pub async fn put_object(
        &self,
        key: &str,
        body: Bytes,
        content_type: Option<&str>,
    ) -> Result<Response<()>, Error> {
        let mut request = Request::new(Method::PUT, Some(key));
        request.body = body;
        if let Some(content_type) = content_type {
            request
                .headers
                .insert("content-type", header(content_type)?);
        }
        self.empty(request).await
    }
    /// Downloads one object into a bounded buffer; default maximum is 32 MiB.
    ///
    /// # Errors
    /// Returns input, credential, transport, size, deadline or COS service errors.
    pub async fn get_object(&self, key: &str) -> Result<Response<Bytes>, Error> {
        self.call(Request::new(Method::GET, Some(key))).await
    }
    /// Requests a closed byte range into a bounded buffer; the server must return HTTP 206.
    ///
    /// # Errors
    /// Returns an error for an invalid range, ignored Range header, or ordinary call failures.
    pub async fn get_object_range(
        &self,
        key: &str,
        range: RangeInclusive<u64>,
    ) -> Result<Response<Bytes>, Error> {
        if range.is_empty() {
            return Err(configuration("invalid COS byte range"));
        }
        let mut request = Request::new(Method::GET, Some(key));
        request.headers.insert(
            "range",
            header(&format!("bytes={}-{}", range.start(), range.end()))?,
        );
        let response = self.call(request).await?;
        if response.status != http::StatusCode::PARTIAL_CONTENT {
            return Err(tencentcloud_sdk_core::SdkError::Protocol {
                message: "COS did not honor the byte range",
                status: response.status,
                request_id: response.request_id,
            }
            .into());
        }
        Ok(response)
    }
    /// Reads object headers, including `ETag`, length, version and user metadata.
    ///
    /// # Errors
    /// Returns ordinary call failures; bodyless HTTP errors retain their status and `RequestId`.
    pub async fn head_object(&self, key: &str) -> Result<Response<()>, Error> {
        self.empty(Request::new(Method::HEAD, Some(key))).await
    }
    /// Deletes the named object once; versioned buckets follow COS's delete-marker semantics.
    ///
    /// # Errors
    /// Returns input, credential, transport, deadline or COS service errors.
    pub async fn delete_object(&self, key: &str) -> Result<Response<()>, Error> {
        self.empty(Request::new(Method::DELETE, Some(key))).await
    }
    /// Copies one object server-side; this does not implement multipart copy for large objects.
    ///
    /// # Errors
    /// Returns input, credential, transport, XML, deadline or COS service errors.
    pub async fn copy_object(
        &self,
        key: &str,
        source_bucket: &str,
        source_region: &str,
        source_key: &str,
    ) -> Result<Response<CopyObjectResult>, Error> {
        validate_bucket(source_bucket)?;
        if source_region.is_empty()
            || !source_region
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(configuration("invalid COS copy source region"));
        }
        let mut request = Request::new(Method::PUT, Some(key));
        let source = format!(
            "{source_bucket}.cos.{source_region}.myqcloud.com{}",
            key_path(source_key)?
        );
        request
            .headers
            .insert("x-cos-copy-source", header(&source)?);
        self.xml(request, "CopyObjectResult").await
    }
    /// Lists one bucket page. Use `NextMarker` explicitly; no hidden pagination requests occur.
    ///
    /// # Errors
    /// Returns an error for invalid limits or ordinary call/XML failures.
    pub async fn list_objects(
        &self,
        options: &ListObjectsOptions,
    ) -> Result<Response<ListObjectsResult>, Error> {
        let mut request = Request::new(Method::GET, None);
        for (key, value) in [
            ("prefix", &options.prefix),
            ("marker", &options.marker),
            ("delimiter", &options.delimiter),
        ] {
            if let Some(value) = value {
                request.query.push((key, value.clone()));
            }
        }
        if let Some(max) = options.max_keys {
            if !(1..=1000).contains(&max) {
                return Err(configuration("COS max_keys must be in 1..=1000"));
            }
            request.query.push(("max-keys", max.to_string()));
        }
        self.xml(request, "ListBucketResult").await
    }
    /// Lists account buckets via the independently configured GET Service endpoint.
    ///
    /// # Errors
    /// Returns credential, transport, XML, deadline or COS service errors.
    pub async fn list_buckets(&self) -> Result<Response<ListBucketsResult>, Error> {
        let mut request = Request::new(Method::GET, None);
        request.service = true;
        self.xml(request, "ListAllMyBucketsResult").await
    }
    /// Creates the configured bucket with COS's default private access policy.
    ///
    /// # Errors
    /// Returns ordinary call failures; this side-effectful operation is never retried.
    pub async fn create_bucket(&self) -> Result<Response<()>, Error> {
        self.empty(Request::new(Method::PUT, None)).await
    }
    /// Checks existence and authorization for the configured bucket.
    ///
    /// # Errors
    /// Returns ordinary call failures, preserving HTTP status for bodyless errors.
    pub async fn head_bucket(&self) -> Result<Response<()>, Error> {
        self.empty(Request::new(Method::HEAD, None)).await
    }
    /// Deletes the configured empty bucket once; objects are never deleted implicitly.
    ///
    /// # Errors
    /// Returns ordinary call failures, including `BucketNotEmpty`.
    pub async fn delete_bucket(&self) -> Result<Response<()>, Error> {
        self.empty(Request::new(Method::DELETE, None)).await
    }
    /// Retrieves the configured bucket's location constraint.
    ///
    /// # Errors
    /// Returns ordinary call or XML failures.
    pub async fn get_bucket_location(&self) -> Result<Response<String>, Error> {
        let mut request = Request::new(Method::GET, None);
        request.query.push(("location", String::new()));
        self.xml(request, "LocationConstraint").await
    }
    /// Initiates multipart upload; the application owns completion or explicit abort.
    ///
    /// # Errors
    /// Returns ordinary call or XML failures; there is no background cleanup/retry.
    pub async fn initiate_multipart_upload(
        &self,
        key: &str,
    ) -> Result<Response<MultipartUpload>, Error> {
        let mut request = Request::new(Method::POST, Some(key));
        request.query.push(("uploads", String::new()));
        self.xml(request, "InitiateMultipartUploadResult").await
    }
    /// Uploads one buffered part, returning its `ETag` header for explicit completion.
    ///
    /// Except for the final part, COS normally requires at least 1 MiB. The server enforces
    /// per-object and part size limits; the client's configured buffer limit also applies.
    ///
    /// # Errors
    /// Returns an error for invalid identifiers/part numbers or ordinary call failures.
    pub async fn upload_part(
        &self,
        key: &str,
        upload_id: &str,
        part_number: u16,
        body: Bytes,
    ) -> Result<Response<()>, Error> {
        validate_part(part_number)?;
        let mut request = multipart(Method::PUT, key, upload_id)?;
        request.query.push(("partNumber", part_number.to_string()));
        request.body = body;
        self.empty(request).await
    }
    /// Lists a page of uploaded parts, continuing after the supplied part-number marker.
    ///
    /// # Errors
    /// Returns validation, ordinary call or XML failures.
    pub async fn list_parts(
        &self,
        key: &str,
        upload_id: &str,
        marker: Option<u16>,
    ) -> Result<Response<ListPartsResult>, Error> {
        let mut request = multipart(Method::GET, key, upload_id)?;
        if let Some(marker) = marker {
            if marker > 10_000 {
                return Err(configuration("invalid COS part-number marker"));
            }
            request
                .query
                .push(("part-number-marker", marker.to_string()));
        }
        self.xml(request, "ListPartsResult").await
    }
    /// Completes an explicitly ordered, nonempty set of parts. Embedded HTTP 200 errors are parsed.
    ///
    /// # Errors
    /// Rejects unordered, duplicate or invalid part numbers/ETags and returns ordinary/XML errors.
    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: &[CompletedPart],
    ) -> Result<Response<CompleteMultipartResult>, Error> {
        if parts.is_empty() || parts.len() > 10_000 {
            return Err(configuration("COS completion requires 1..=10000 parts"));
        }
        let mut previous = 0;
        for part in parts {
            validate_part(part.part_number)?;
            if part.part_number <= previous
                || part.etag.is_empty()
                || part.etag.len() > 256
                || part.etag.chars().any(char::is_control)
            {
                return Err(configuration(
                    "COS completion parts must be ordered with valid ETags",
                ));
            }
            previous = part.part_number;
        }
        let body = quick_xml::se::to_string(&Complete { parts })
            .map_err(|_| configuration("cannot encode COS multipart completion"))?;
        let mut request = multipart(Method::POST, key, upload_id)?;
        request
            .headers
            .insert("content-type", header("application/xml")?);
        request.body = Bytes::from(body);
        self.xml(request, "CompleteMultipartUploadResult").await
    }
    /// Aborts this multipart upload once; never called implicitly after a failure.
    ///
    /// # Errors
    /// Returns validation or ordinary call failures.
    pub async fn abort_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
    ) -> Result<Response<()>, Error> {
        self.empty(multipart(Method::DELETE, key, upload_id)?).await
    }
}
fn multipart<'a>(method: Method, key: &'a str, upload_id: &str) -> Result<Request<'a>, Error> {
    if upload_id.is_empty() || upload_id.len() > 2048 || upload_id.chars().any(char::is_control) {
        return Err(configuration("invalid COS upload identifier"));
    }
    let mut request = Request::new(method, Some(key));
    request.query.push(("uploadId", upload_id.to_owned()));
    Ok(request)
}
fn validate_part(number: u16) -> Result<(), Error> {
    if !(1..=10_000).contains(&number) {
        return Err(configuration("COS part number must be in 1..=10000"));
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename = "CompleteMultipartUpload")]
struct Complete<'a> {
    #[serde(rename = "Part")]
    parts: &'a [CompletedPart],
}
