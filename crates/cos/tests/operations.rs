//! Offline protocol tests using fake credentials and no ambient cloud configuration.
use bytes::Bytes;
use futures_util::stream;
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use std::{
    future::{Future, pending},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tencentcloud_sdk_cos::{
    Client, ClientBuilder, Error,
    models::{CompletedPart, ListObjectsOptions},
};
use tencentcloud_sdk_foundation::{
    BoxFuture, Credentials, Endpoint, SdkError,
    credentials::ProvideCredentials,
    runtime::Clock,
    transport::{Transport, TransportRequest, TransportResponse},
};

type Handler = dyn Fn(TransportRequest) -> BoxFuture<'static, Result<TransportResponse, SdkError>>
    + Send
    + Sync;
#[derive(Clone)]
struct Mock {
    handler: Arc<Handler>,
    calls: Arc<AtomicUsize>,
}
impl Mock {
    fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(TransportRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<TransportResponse, SdkError>> + Send + 'static,
    {
        Self {
            handler: Arc::new(move |request| Box::pin(handler(request))),
            calls: Arc::new(AtomicUsize::new(0)),
        }
    }
    fn count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl Transport for Mock {
    fn send(
        &self,
        request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        (self.handler)(request)
    }
}
struct FixedClock;
impl Clock for FixedClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(1_700_000_000)
    }
}
fn builder(mock: &Mock) -> ClientBuilder {
    Client::builder("bucket-123", "ap-shanghai")
        .credentials(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("test-token")
                .unwrap(),
        )
        .shared_transport(Arc::new(mock.clone()))
        .clock(FixedClock)
        .endpoint(Endpoint::loopback_http("http://127.0.0.1:12345/").unwrap())
        .service_endpoint(Endpoint::loopback_http("http://127.0.0.1:12346/").unwrap())
}
fn reply(status: StatusCode, body: impl Into<Bytes>) -> TransportResponse {
    let mut headers = HeaderMap::new();
    headers.insert("x-cos-request-id", HeaderValue::from_static("cos-request"));
    headers.insert("etag", HeaderValue::from_static("\"opaque-etag\""));
    TransportResponse::from_bytes(status, headers, body)
}
fn signed(request: &TransportRequest) {
    assert_eq!(request.headers["x-cos-security-token"], "test-token");
    assert!(request.headers["x-cos-security-token"].is_sensitive());
    let auth = &request.headers["authorization"];
    assert!(auth.is_sensitive());
    assert!(
        auth.to_str()
            .unwrap()
            .starts_with("q-sign-algorithm=sha1&q-ak=test-id")
    );
    assert!(!request.headers.contains_key("x-tc-action"));
}

#[tokio::test]
async fn object_upload_download_range_head_and_delete_preserve_wire_bytes() {
    let mock = Mock::new(|request| async move {
        signed(&request);
        assert_eq!(request.uri.path(), "/folder/%E4%B8%AD%20%2B%25.txt");
        match request.method {
            Method::PUT => {
                assert_eq!(request.body, "hello");
                assert_eq!(request.headers["content-type"], "text/plain");
                assert_eq!(request.headers["content-length"], "5");
                assert_eq!(request.headers["content-md5"], "XUFAKrxLKna5cZ2REBfFkg==");
                Ok(reply(StatusCode::OK, Bytes::new()))
            }
            Method::GET => {
                if request.headers.contains_key("range") {
                    assert_eq!(request.headers["range"], "bytes=1-3");
                    Ok(reply(StatusCode::PARTIAL_CONTENT, "ell"))
                } else {
                    Ok(reply(StatusCode::OK, "hello"))
                }
            }
            Method::HEAD | Method::DELETE => Ok(reply(StatusCode::NO_CONTENT, Bytes::new())),
            _ => panic!("unexpected method"),
        }
    });
    let client = builder(&mock).build().unwrap();
    let key = "folder/中 +%.txt";
    let upload = client
        .put_object(key, Bytes::from_static(b"hello"), Some("text/plain"))
        .await
        .unwrap();
    assert_eq!(upload.headers["etag"], "\"opaque-etag\"");
    assert_eq!(upload.request_id.as_deref(), Some("cos-request"));
    let object = client.get_object(key).await.unwrap();
    assert_eq!(object.data, "hello");
    assert!(!format!("{object:?}").contains("hello"));
    assert_eq!(
        client.get_object_range(key, 1..=3).await.unwrap().data,
        "ell"
    );
    client.head_object(key).await.unwrap();
    client.delete_object(key).await.unwrap();
    assert_eq!(mock.count(), 5);
}

#[tokio::test]
async fn bucket_operations_and_single_page_listing_use_distinct_service_root() {
    let mock = Mock::new(|request| async move {
        signed(&request);
        assert_eq!(request.uri.path(), "/");
        if request.uri.port_u16() == Some(12346) {
            return Ok(reply(
                StatusCode::OK,
                "<ListAllMyBucketsResult><Buckets><Bucket><Name>bucket-123</Name><Location>ap-shanghai</Location><CreationDate>2020-01-01</CreationDate></Bucket></Buckets></ListAllMyBucketsResult>",
            ));
        }
        match (request.method, request.uri.query()) {
            (Method::GET, Some("location=")) => Ok(reply(
                StatusCode::OK,
                "<LocationConstraint>ap-shanghai</LocationConstraint>",
            )),
            (Method::GET, Some(query)) => {
                assert_eq!(
                    query,
                    "prefix=%E4%B8%AD%2F&marker=a%2Bb&delimiter=%2F&max-keys=2"
                );
                Ok(reply(
                    StatusCode::OK,
                    "<?xml version=\"1.0\"?><ListBucketResult><Name>bucket-123</Name><IsTruncated>true</IsTruncated><NextMarker>next-key</NextMarker><Contents><Key>中/object</Key><LastModified>now</LastModified><ETag>&quot;tag&quot;</ETag><Size>42</Size><StorageClass>STANDARD</StorageClass></Contents><CommonPrefixes><Prefix>中/folder/</Prefix></CommonPrefixes></ListBucketResult>",
                ))
            }
            (Method::HEAD | Method::PUT | Method::DELETE, None) => {
                Ok(reply(StatusCode::OK, Bytes::new()))
            }
            _ => panic!("unexpected bucket request"),
        }
    });
    let client = builder(&mock).build().unwrap();
    client.create_bucket().await.unwrap();
    client.head_bucket().await.unwrap();
    client.delete_bucket().await.unwrap();
    assert_eq!(
        client.get_bucket_location().await.unwrap().data,
        "ap-shanghai"
    );
    let listed = client
        .list_objects(&ListObjectsOptions {
            prefix: Some("中/".into()),
            marker: Some("a+b".into()),
            delimiter: Some("/".into()),
            max_keys: Some(2),
        })
        .await
        .unwrap()
        .data;
    assert!(listed.is_truncated);
    assert_eq!(listed.next_marker.as_deref(), Some("next-key"));
    assert_eq!(listed.objects[0].size, 42);
    assert_eq!(listed.objects[0].key, "中/object");
    assert_eq!(listed.objects[0].etag, "\"tag\"");
    assert_eq!(listed.common_prefixes[0].prefix, "中/folder/");
    let buckets = client.list_buckets().await.unwrap().data;
    assert_eq!(buckets.buckets.items[0].name, "bucket-123");
    assert_eq!(mock.count(), 6);
}

#[tokio::test]
async fn copy_and_multipart_lifecycle_encode_reserved_identifiers_and_xml_parts() {
    let mock = Mock::new(|request| async move {
        signed(&request);
        assert_eq!(request.uri.path(), "/destination");
        if request.headers.contains_key("x-cos-copy-source") {
            assert_eq!(
                request.headers["x-cos-copy-source"],
                "source-456.cos.ap-beijing.myqcloud.com/a%20b/%E4%B8%AD"
            );
            return Ok(reply(
                StatusCode::OK,
                "<CopyObjectResult><ETag>copy-tag</ETag><LastModified>now</LastModified></CopyObjectResult>",
            ));
        }
        match (request.method, request.uri.query().unwrap()) {
            (Method::POST, "uploads=") => Ok(reply(
                StatusCode::OK,
                "<InitiateMultipartUploadResult><Bucket>bucket-123</Bucket><Key>destination</Key><UploadId>id+/=</UploadId></InitiateMultipartUploadResult>",
            )),
            (Method::PUT, "uploadId=id%2B%2F%3D&partNumber=1") => {
                assert_eq!(request.body, "part-body");
                assert!(request.headers.contains_key("content-md5"));
                Ok(reply(StatusCode::OK, Bytes::new()))
            }
            (Method::GET, "uploadId=id%2B%2F%3D&part-number-marker=0") => Ok(reply(
                StatusCode::OK,
                "<ListPartsResult><Bucket>bucket-123</Bucket><Key>destination</Key><UploadId>id+/=</UploadId><IsTruncated>false</IsTruncated><NextPartNumberMarker>1</NextPartNumberMarker><Part><PartNumber>1</PartNumber><ETag>&quot;part&amp;tag&quot;</ETag></Part></ListPartsResult>",
            )),
            (Method::POST, "uploadId=id%2B%2F%3D") => {
                let xml = std::str::from_utf8(&request.body).unwrap();
                assert!(xml.starts_with("<CompleteMultipartUpload>"));
                assert!(xml.contains("<PartNumber>1</PartNumber>"));
                assert!(xml.contains("&amp;"));
                assert_eq!(request.headers["content-type"], "application/xml");
                Ok(reply(
                    StatusCode::OK,
                    "<CompleteMultipartUploadResult><Location>location</Location><Bucket>bucket-123</Bucket><Key>destination</Key><ETag>complete-tag</ETag></CompleteMultipartUploadResult>",
                ))
            }
            (Method::DELETE, "uploadId=id%2B%2F%3D") => {
                Ok(reply(StatusCode::NO_CONTENT, Bytes::new()))
            }
            _ => panic!("unexpected multipart operation"),
        }
    });
    let client = builder(&mock).build().unwrap();
    assert_eq!(
        client
            .copy_object("destination", "source-456", "ap-beijing", "a b/中")
            .await
            .unwrap()
            .data
            .etag,
        "copy-tag"
    );
    let upload = client
        .initiate_multipart_upload("destination")
        .await
        .unwrap()
        .data;
    assert_eq!(upload.upload_id, "id+/=");
    client
        .upload_part(
            "destination",
            &upload.upload_id,
            1,
            Bytes::from_static(b"part-body"),
        )
        .await
        .unwrap();
    let parts = client
        .list_parts("destination", &upload.upload_id, Some(0))
        .await
        .unwrap()
        .data;
    assert_eq!(parts.parts[0].etag, "\"part&tag\"");
    assert_eq!(
        client
            .complete_multipart_upload("destination", &upload.upload_id, &parts.parts)
            .await
            .unwrap()
            .data
            .etag,
        "complete-tag"
    );
    client
        .abort_multipart_upload("destination", &upload.upload_id)
        .await
        .unwrap();
    assert_eq!(mock.count(), 6);
}

#[tokio::test]
async fn presigned_capabilities_are_redacted_and_never_sent_by_the_sdk() {
    let mock = Mock::new(|_| async { panic!("presigning must not send requests") });
    let client = builder(&mock).build().unwrap();
    for method in [Method::GET, Method::HEAD, Method::PUT, Method::DELETE] {
        let url = client
            .presign(method, "中 +%.txt", Duration::from_secs(60))
            .await
            .unwrap();
        assert!(
            url.as_str()
                .contains("/%E4%B8%AD%20%2B%25.txt?x-cos-security-token=test-token&")
        );
        assert!(url.as_str().contains("q-header-list=host"));
        assert!(!url.as_str().contains("content-length"));
        assert!(!format!("{url:?}").contains("test-token"));
    }
    assert!(
        client
            .presign(Method::POST, "key", Duration::from_secs(60))
            .await
            .is_err()
    );
    assert!(
        client
            .presign(Method::GET, "key", Duration::ZERO)
            .await
            .is_err()
    );
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn invalid_inputs_fail_before_transport() {
    let mock = Mock::new(|_| async { panic!("invalid input must not reach transport") });
    let client = builder(&mock).max_request_body(3).build().unwrap();
    for key in ["", ".", "a/../b", "a/./b", "bad\nkey", &"a".repeat(851)] {
        assert!(client.get_object(key).await.is_err());
    }
    assert!(
        client
            .put_object("key", Bytes::from_static(b"four"), None)
            .await
            .is_err()
    );
    assert!(
        client
            .put_object("key", Bytes::new(), Some("bad\nheader"))
            .await
            .is_err()
    );
    assert!(
        client
            .list_objects(&ListObjectsOptions {
                max_keys: Some(0),
                ..Default::default()
            })
            .await
            .is_err()
    );
    assert!(
        client
            .get_object_range("key", RangeInclusive::new(4, 2))
            .await
            .is_err()
    );
    assert!(
        client
            .copy_object("key", "bad", "ap-shanghai", "source")
            .await
            .is_err()
    );
    assert!(
        client
            .copy_object("key", "source-123", "bad/region", "source")
            .await
            .is_err()
    );
    for number in [0, 10_001] {
        assert!(
            client
                .upload_part("key", "id", number, Bytes::new())
                .await
                .is_err()
        );
    }
    assert!(client.abort_multipart_upload("key", "").await.is_err());
    assert!(client.list_parts("key", "id", Some(10_001)).await.is_err());
    assert!(
        client
            .complete_multipart_upload("key", "id", &[])
            .await
            .is_err()
    );
    for parts in [
        vec![CompletedPart {
            part_number: 0,
            etag: "tag".into(),
        }],
        vec![CompletedPart {
            part_number: 1,
            etag: String::new(),
        }],
        vec![
            CompletedPart {
                part_number: 2,
                etag: "tag".into(),
            },
            CompletedPart {
                part_number: 1,
                etag: "tag".into(),
            },
        ],
    ] {
        assert!(
            client
                .complete_multipart_upload("key", "id", &parts)
                .await
                .is_err()
        );
    }
    assert_eq!(mock.count(), 0);
}
use std::ops::RangeInclusive;

#[test]
fn builders_reject_invalid_configuration_without_network_access() {
    for (bucket, region) in [
        ("missing-appid", "ap-shanghai"),
        ("-bad-123", "ap-shanghai"),
        ("UPPER-123", "ap-shanghai"),
        ("bucket-123", ""),
        ("bucket-123", "../bad"),
    ] {
        assert!(Client::builder(bucket, region).build().is_err());
    }
    assert!(
        Client::builder("bucket-123", "ap-shanghai")
            .build()
            .is_err()
    );
    let mock = Mock::new(|_| async { panic!("building must not send requests") });
    assert!(builder(&mock).timeout(Duration::ZERO).build().is_err());
    assert!(builder(&mock).max_request_body(0).build().is_err());
    assert!(builder(&mock).max_response_body(0).build().is_err());
    #[cfg(feature = "transport-reqwest")]
    Client::builder("bucket-123", "ap-shanghai")
        .credentials(Credentials::new("id", "key").unwrap())
        .build()
        .unwrap();
    #[cfg(not(feature = "transport-reqwest"))]
    assert!(
        Client::builder("bucket-123", "ap-shanghai")
            .credentials(Credentials::new("id", "key").unwrap())
            .build()
            .is_err()
    );
}

#[tokio::test]
async fn xml_service_errors_preserve_codes_request_ids_and_never_retry() {
    for status in [StatusCode::FORBIDDEN, StatusCode::OK] {
        let mock = Mock::new(move |_| async move {
            Ok(reply(
                status,
                "<Error><Code>AccessDenied</Code><Message>private-object-detail</Message><RequestId>xml-request</RequestId></Error>",
            ))
        });
        let client = builder(&mock).build().unwrap();
        let parts = [CompletedPart {
            part_number: 1,
            etag: "tag".into(),
        }];
        let error = client
            .complete_multipart_upload("key", "id", &parts)
            .await
            .unwrap_err();
        assert!(!format!("{error:?}").contains("private-object-detail"));
        assert!(!error.to_string().contains("private-object-detail"));
        assert!(std::error::Error::source(&error).is_some());
        match error {
            Error::Service(service) => {
                assert_eq!(service.code, "AccessDenied");
                assert_eq!(service.request_id.as_deref(), Some("xml-request"));
                assert_eq!(service.status, status);
                assert_eq!(service.message, "private-object-detail");
                assert!(!format!("{service:?}").contains("private-object-detail"));
            }
            error => panic!("unexpected error: {error}"),
        }
        assert_eq!(mock.count(), 1);
    }
    let mock = Mock::new(|_| async {
        Ok(reply(
            StatusCode::NOT_FOUND,
            "<Error><Code>NoSuchKey</Code><Message>missing</Message></Error>",
        ))
    });
    match builder(&mock)
        .build()
        .unwrap()
        .get_object("key")
        .await
        .unwrap_err()
    {
        Error::Service(service) => assert_eq!(service.request_id.as_deref(), Some("cos-request")),
        error => panic!("unexpected error: {error}"),
    }
}

#[tokio::test]
async fn malformed_xml_unexpected_bodies_and_bodyless_errors_are_protocol_errors() {
    for (status, body) in [
        (StatusCode::OK, ""),
        (StatusCode::OK, "not XML"),
        (StatusCode::OK, "<Unexpected/>"),
        (StatusCode::OK, "<ListBucketResult>"),
        (StatusCode::OK, "<Error/>"),
        (
            StatusCode::OK,
            "<!DOCTYPE x [<!ENTITY s SYSTEM 'file:///not-read'>]><ListBucketResult/>",
        ),
        (
            StatusCode::OK,
            "<?xml version='1.0'?><!-- ok --><?test?><ListBucketResult><Name>bucket</Name><IsTruncated>not-bool</IsTruncated></ListBucketResult>",
        ),
        (StatusCode::BAD_GATEWAY, "upstream HTML"),
    ] {
        let mock = Mock::new(move |_| async move { Ok(reply(status, body)) });
        let error = builder(&mock)
            .build()
            .unwrap()
            .list_objects(&ListObjectsOptions::default())
            .await
            .unwrap_err();
        match error {
            Error::Sdk(SdkError::Protocol {
                status: actual,
                request_id,
                ..
            }) => {
                assert_eq!(actual, status);
                assert_eq!(request_id.as_deref(), Some("cos-request"));
            }
            error => panic!("unexpected error: {error}"),
        }
    }
    let mock = Mock::new(|_| async { Ok(reply(StatusCode::FORBIDDEN, Bytes::new())) });
    assert!(matches!(
        builder(&mock).build().unwrap().head_object("key").await,
        Err(Error::Sdk(SdkError::Protocol {
            status: StatusCode::FORBIDDEN,
            ..
        }))
    ));
    let mock = Mock::new(|_| async { Ok(reply(StatusCode::OK, "unexpected-body")) });
    assert!(
        builder(&mock)
            .build()
            .unwrap()
            .delete_object("key")
            .await
            .is_err()
    );
    assert!(
        builder(&mock)
            .build()
            .unwrap()
            .get_object_range("key", 0..=1)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn bounded_body_reads_use_actual_chunks_not_content_length() {
    let mock = Mock::new(|_| async {
        let mut headers = HeaderMap::new();
        headers.insert("content-length", HeaderValue::from_static("1"));
        Ok(TransportResponse {
            status: StatusCode::OK,
            headers,
            body: Box::pin(stream::iter([
                Ok(Bytes::new()),
                Ok(Bytes::from_static(b"ab")),
                Ok(Bytes::from_static(b"cd")),
            ])),
        })
    });
    let error = builder(&mock)
        .max_response_body(3)
        .build()
        .unwrap()
        .get_object("key")
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        Error::Sdk(SdkError::BodyTooLarge { limit: 3 })
    ));
    let mock = Mock::new(|_| async {
        Ok(TransportResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Box::pin(stream::iter([Err(SdkError::Transport(Box::new(
                std::io::Error::other("private-source"),
            )))])),
        })
    });
    let error = builder(&mock)
        .build()
        .unwrap()
        .get_object("key")
        .await
        .unwrap_err();
    assert!(std::error::Error::source(&error).is_some());
    assert!(!format!("{error:?}").contains("private-source"));
}

struct NeverCredentials;
impl ProvideCredentials for NeverCredentials {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        Box::pin(pending())
    }
}
#[tokio::test(start_paused = true)]
async fn deadline_covers_credentials_send_and_body_without_replay() {
    let mock = Mock::new(|_| async { panic!("credentials must time out before transport") });
    let client = builder(&mock)
        .credentials_provider(NeverCredentials)
        .timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    assert!(matches!(
        client.get_object("key").await,
        Err(Error::Sdk(SdkError::Timeout))
    ));
    assert!(matches!(
        client
            .presign(Method::GET, "key", Duration::from_secs(60))
            .await,
        Err(Error::Sdk(SdkError::Timeout))
    ));
    assert_eq!(mock.count(), 0);
    let mock = Mock::new(|_| pending());
    assert!(matches!(
        builder(&mock)
            .timeout(Duration::from_millis(10))
            .build()
            .unwrap()
            .get_object("key")
            .await,
        Err(Error::Sdk(SdkError::Timeout))
    ));
    assert_eq!(mock.count(), 1);
    let mock = Mock::new(|_| async {
        Ok(TransportResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Box::pin(stream::pending()),
        })
    });
    assert!(matches!(
        builder(&mock)
            .timeout(Duration::from_millis(10))
            .build()
            .unwrap()
            .get_object("key")
            .await,
        Err(Error::Sdk(SdkError::Timeout))
    ));
    assert_eq!(mock.count(), 1);
}
