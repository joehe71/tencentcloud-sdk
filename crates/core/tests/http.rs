//! HTTP adapter tests use local mock servers on numeric loopback addresses, never Tencent Cloud.

#![cfg(feature = "transport-reqwest")]

use std::{io, time::Duration};

use serde_json::{Value, json};
use tencentcloud_sdk_foundation::{
    ApiResponse, Client, Credentials, Endpoint, Operation, SdkError,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
    time::timeout,
};

async fn read_request(stream: &mut BufReader<TcpStream>) -> io::Result<(String, Vec<u8>)> {
    let mut headers = String::new();
    let mut length = 0;
    loop {
        let mut line = String::new();
        if stream.read_line(&mut line).await? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "request ended prematurely",
            ));
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                length = value
                    .trim()
                    .parse()
                    .map_err(|_| io::Error::other("invalid request length"))?;
            }
        }
        let done = line == "\r\n";
        headers.push_str(&line);
        if done {
            break;
        }
    }
    let mut body = vec![0; length];
    stream.read_exact(&mut body).await?;
    Ok((headers, body))
}

fn client(address: std::net::SocketAddr) -> Client {
    Client::builder()
        .credentials(Credentials::new("test-id", "test-key").unwrap())
        .endpoint(
            "cvm",
            Endpoint::loopback_http(&format!("http://{address}/")).unwrap(),
        )
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap()
}

async fn call(client: &Client) -> Result<ApiResponse<Value>, SdkError> {
    let operation = Operation::new("cvm", "2017-03-12", "DescribeInstances").unwrap();
    client.execute(&operation, &json!({"Limit": 1})).await
}

async fn join<T: Send + 'static>(task: JoinHandle<T>) -> T {
    timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
        .unwrap()
}

#[tokio::test]
async fn reqwest_reuses_one_connection_and_sends_signed_bytes_unchanged() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut stream = BufReader::new(socket);
        let mut requests = Vec::new();
        for _ in 0..2 {
            requests.push(read_request(&mut stream).await.unwrap());
            let body = r#"{"Response":{"RequestId":"local-request"}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            );
            stream
                .get_mut()
                .write_all(response.as_bytes())
                .await
                .unwrap();
            stream.get_mut().flush().await.unwrap();
        }
        requests
    });
    let client = client(address);
    assert_eq!(
        call(&client).await.unwrap().metadata.request_id.as_deref(),
        Some("local-request")
    );
    assert!(call(&client.clone()).await.is_ok());
    for (headers, body) in join(server).await {
        assert!(headers.starts_with("POST / HTTP/1.1\r\n"));
        let lower = headers.to_ascii_lowercase();
        assert!(lower.contains(&format!("host: {address}\r\n")));
        assert!(lower.contains("authorization: tc3-hmac-sha256"));
        assert!(lower.contains("x-tc-action: describeinstances\r\n"));
        assert_eq!(body, br#"{"Limit":1}"#);
    }
}

#[tokio::test]
async fn redirect_is_returned_without_forwarding_credentials() {
    let target = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target_address = target.local_addr().unwrap();
    let source = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = source.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = source.accept().await.unwrap();
        let mut stream = BufReader::new(socket);
        read_request(&mut stream).await.unwrap();
        let response = format!(
            "HTTP/1.1 302 Found\r\nLocation: http://{target_address}/\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        );
        stream
            .get_mut()
            .write_all(response.as_bytes())
            .await
            .unwrap();
    });
    assert!(
        matches!(call(&client(address)).await, Err(SdkError::Protocol { status, .. }) if status.as_u16() == 302)
    );
    join(server).await;
    assert!(
        timeout(Duration::from_millis(50), target.accept())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn non_json_http_error_keeps_request_id_and_status() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut stream = BufReader::new(socket);
        read_request(&mut stream).await.unwrap();
        stream.get_mut().write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Type: text/html\r\nX-TC-RequestId: proxy-id\r\nContent-Length: 7\r\nConnection: close\r\n\r\nprivate").await.unwrap();
    });
    let error = call(&client(address)).await.unwrap_err();
    assert!(!format!("{error:?}").contains("private"));
    assert!(
        matches!(error, SdkError::Protocol { status, request_id: Some(id), .. }
        if status.as_u16() == 502 && id == "proxy-id")
    );
    join(server).await;
}

#[tokio::test]
async fn body_limit_is_enforced_on_http_without_content_length() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut stream = BufReader::new(socket);
        read_request(&mut stream).await.unwrap();
        stream.get_mut().write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n4\r\n1234\r\n4\r\n5678\r\n0\r\n\r\n").await.unwrap();
    });
    let client = Client::builder()
        .credentials(Credentials::new("test-id", "test-key").unwrap())
        .endpoint(
            "cvm",
            Endpoint::loopback_http(&format!("http://{address}/")).unwrap(),
        )
        .max_response_body(7)
        .build()
        .unwrap();
    assert!(matches!(
        call(&client).await,
        Err(SdkError::BodyTooLarge { limit: 7 })
    ));
    join(server).await;
}
