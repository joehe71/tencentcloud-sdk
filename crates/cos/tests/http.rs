//! The real HTTP adapter talks only to a numeric loopback server, never COS.
#![cfg(feature = "transport-reqwest")]
use bytes::Bytes;
use std::time::Duration;
use tencentcloud_sdk_cos::Client;
use tencentcloud_sdk_foundation::{Credentials, Endpoint};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    time::timeout,
};

#[tokio::test]
async fn reqwest_keeps_encoded_keys_tokens_checksums_and_connection_pool_intact() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut stream = BufReader::new(socket);
        for method in ["PUT", "GET"] {
            let mut first = String::new();
            stream.read_line(&mut first).await.unwrap();
            assert_eq!(
                first,
                format!("{method} /folder/%E4%B8%AD%20%2B%25 HTTP/1.1\r\n")
            );
            let mut headers = String::new();
            let mut length = 0;
            loop {
                let mut line = String::new();
                assert_ne!(stream.read_line(&mut line).await.unwrap(), 0);
                if let Some((name, value)) = line.split_once(':') {
                    if name.eq_ignore_ascii_case("content-length") {
                        length = value.trim().parse::<usize>().unwrap();
                    }
                }
                headers.push_str(&line.to_ascii_lowercase());
                if line == "\r\n" {
                    break;
                }
            }
            let mut body = vec![0; length];
            stream.read_exact(&mut body).await.unwrap();
            assert!(headers.contains("authorization: q-sign-algorithm=sha1"));
            assert!(headers.contains("x-cos-security-token: test-token\r\n"));
            assert!(headers.contains(&format!("host: {address}\r\n")));
            let response_body = if method == "PUT" {
                assert_eq!(body, b"hello");
                assert!(headers.contains("content-md5: xufakrxlkna5cz2rebffkg==\r\n"));
                ""
            } else {
                // An object whose content happens to be XML Error is still an ordinary object.
                "<Error><Code>stored-object</Code></Error>"
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nx-cos-request-id: loopback-id\r\n\r\n{response_body}",
                response_body.len()
            );
            stream
                .get_mut()
                .write_all(response.as_bytes())
                .await
                .unwrap();
            stream.get_mut().flush().await.unwrap();
        }
    });
    let client = Client::builder("bucket-123", "ap-shanghai")
        .credentials(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("test-token")
                .unwrap(),
        )
        .endpoint(Endpoint::loopback_http(&format!("http://{address}/")).unwrap())
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();
    client
        .put_object(
            "folder/中 +%",
            Bytes::from_static(b"hello"),
            Some("text/plain"),
        )
        .await
        .unwrap();
    let result = client.clone().get_object("folder/中 +%").await.unwrap();
    assert_eq!(result.request_id.as_deref(), Some("loopback-id"));
    assert_eq!(result.data, "<Error><Code>stored-object</Code></Error>");
    timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
