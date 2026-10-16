//! Explicitly invoked offline benchmarks of actual core implementations without public test APIs.

use std::{hint::black_box, path::Path, time::Duration};

use bytes::Bytes;
use criterion::{BenchmarkId, Criterion, Throughput};
use http::{HeaderMap, HeaderValue, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::{
    ApiResponse, BoxFuture, Client, Credentials, Operation, SdkError,
    credentials::{ProvideCredentials, StaticCredentialsProvider},
    protocol::{self, RawResponse},
    runtime::Clock,
    signing,
    transport::{Transport, TransportRequest, TransportResponse},
};

const SIZES: [usize; 3] = [1024, 64 * 1024, 1024 * 1024];
const LIMIT: usize = 32 * 1024 * 1024;

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    #[serde(rename = "Data")]
    data: String,
}

fn request(size: usize) -> Payload {
    Payload {
        data: "x".repeat(size - br#"{"Data":""}"#.len()),
    }
}

fn response_body(size: usize) -> Bytes {
    let overhead = br#"{"Response":{"Data":"","RequestId":"bench-request"}}"#.len();
    Bytes::from(format!(
        r#"{{"Response":{{"Data":"{}","RequestId":"bench-request"}}}}"#,
        "x".repeat(size - overhead)
    ))
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    headers
}

fn credentials() -> Credentials {
    Credentials::new("bench-id", "bench-key").unwrap()
}

fn runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
}

struct FixedClock;
impl Clock for FixedClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        Ok(1_551_113_065)
    }
}

struct MockTransport {
    body: Bytes,
    headers: HeaderMap,
}
impl Transport for MockTransport {
    fn send(
        &self,
        _request: TransportRequest,
    ) -> BoxFuture<'_, Result<TransportResponse, SdkError>> {
        Box::pin(async {
            Ok(TransportResponse::from_bytes(
                StatusCode::OK,
                self.headers.clone(),
                self.body.clone(),
            ))
        })
    }
}

fn client(response_size: usize) -> Client {
    Client::builder()
        .credentials(credentials())
        .clock(FixedClock)
        .region("ap-shanghai")
        .transport(MockTransport {
            body: response_body(response_size),
            headers: headers(),
        })
        .build()
        .unwrap()
}

fn encode_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("encode");
    for size in SIZES {
        let input = request(size);
        assert_eq!(protocol::encode(&input, LIMIT).unwrap().len(), size);
        group.throughput(Throughput::Bytes(u64::try_from(size).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &input,
            |bencher, input| {
                bencher.iter(|| protocol::encode(black_box(input), LIMIT).unwrap());
            },
        );
    }
    group.finish();
}

fn signing_benchmarks(criterion: &mut Criterion) {
    let credentials = credentials();
    let mut group = criterion.benchmark_group("tc3_sign");
    for size in SIZES {
        let body = protocol::encode(&request(size), LIMIT).unwrap();
        group.throughput(Throughput::Bytes(u64::try_from(size).unwrap()));
        group.bench_with_input(BenchmarkId::from_parameter(size), &body, |bencher, body| {
            bencher.iter(|| {
                signing::authorization(
                    black_box(&credentials),
                    "cvm",
                    "cvm.tencentcloudapi.com",
                    "DescribeInstances",
                    black_box(body),
                    black_box(1_551_113_065),
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

fn decode_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("decode");
    for size in SIZES {
        let response = RawResponse {
            status: StatusCode::OK,
            headers: headers(),
            body: response_body(size),
        };
        assert_eq!(response.body.len(), size);
        let decoded: ApiResponse<Payload> = protocol::decode(&response).unwrap();
        assert_ne!(decoded.data.data, "");
        group.throughput(Throughput::Bytes(u64::try_from(size).unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &response,
            |bencher, response| {
                bencher.iter(|| protocol::decode::<Payload>(black_box(response)).unwrap());
            },
        );
    }
    group.finish();
    let error = RawResponse { status: StatusCode::TOO_MANY_REQUESTS, headers: headers(),
        body: Bytes::from_static(br#"{"Response":{"Error":{"Code":"RequestLimitExceeded","Message":"mock limit"},"RequestId":"bench-error"}}"#) };
    assert!(matches!(
        protocol::decode::<Payload>(&error),
        Err(SdkError::Service(_))
    ));
    criterion.bench_function("decode_service_error", |bencher| {
        bencher.iter(|| protocol::decode::<Payload>(black_box(&error)).unwrap_err());
    });
}

fn execute_benchmarks(criterion: &mut Criterion) {
    let runtime = runtime();
    let operation = Operation::new("cvm", "2017-03-12", "DescribeInstances").unwrap();
    let mut group = criterion.benchmark_group("mock_execute");
    for size in SIZES {
        let client = client(size);
        let input = request(size);
        let verified: ApiResponse<Payload> = runtime
            .block_on(client.execute(&operation, &input))
            .unwrap();
        assert_eq!(
            verified.metadata.request_id.as_deref(),
            Some("bench-request")
        );
        group.throughput(Throughput::Bytes(u64::try_from(size * 2).unwrap()));
        group.bench_function(BenchmarkId::from_parameter(size), |bencher| {
            bencher.iter(|| {
                runtime
                    .block_on(
                        client.execute::<_, Payload>(black_box(&operation), black_box(&input)),
                    )
                    .unwrap()
            });
        });
    }
    group.finish();
}

fn shared_resource_benchmarks(criterion: &mut Criterion) {
    let client = client(1024);
    criterion.bench_function("client_clone", |bencher| {
        bencher.iter(|| black_box(&client).clone());
    });
    let runtime = runtime();
    let provider = StaticCredentialsProvider::new(credentials());
    criterion.bench_function("static_credentials", |bencher| {
        bencher.iter(|| {
            runtime
                .block_on(black_box(&provider).provide_credentials())
                .unwrap()
        });
    });
}

#[cfg(debug_assertions)]
fn require_release() {
    panic!("run performance benchmarks with --release");
}

#[cfg(not(debug_assertions))]
fn require_release() {}

#[test]
#[ignore = "offline benchmarks; require --release and explicit --ignored; excluded from default test runs"]
fn core_performance_baseline() {
    require_release();
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/criterion-core");
    // Do not parse libtest arguments as Criterion CLI options; flags such as --ignored would conflict.
    let mut criterion = Criterion::default()
        .without_plots()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2))
        .nresamples(10_000)
        .confidence_level(0.95)
        .output_directory(&directory);
    encode_benchmarks(&mut criterion);
    signing_benchmarks(&mut criterion);
    decode_benchmarks(&mut criterion);
    execute_benchmarks(&mut criterion);
    shared_resource_benchmarks(&mut criterion);
    criterion.final_summary();
}
