use relay_lib::server;
use relay_lib::state::AppState;
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[tokio::test]
async fn test_full_server_api_and_clipboard() {
    let app_state = AppState::new();
    let temp_shared = std::env::temp_dir().join("relay_test_full_suite");
    let _ = fs::create_dir_all(&temp_shared);
    fs::write(temp_shared.join("document.pdf"), b"sample pdf content").unwrap();

    {
        let mut inner = app_state.inner.lock().await;
        inner.shared_folder = temp_shared.clone();
    }

    let port = 19420;
    let (tx, rx) = oneshot::channel();
    let state_clone = app_state.clone();

    let server_handle = tokio::spawn(async move {
        server::run_server(state_clone, port, rx).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

    // 1. Test GET /api/files
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream
        .write_all(b"GET /api/files HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("document.pdf"));

    // 2. Test POST /api/files/mkdir
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    let body = r#"{"path":"TestFolder"}"#;
    let req = format!(
        "POST /api/files/mkdir HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(), body
    );
    stream.write_all(req.as_bytes()).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("201 Created") || resp_str.contains("200 OK"));
    assert!(temp_shared.join("TestFolder").exists());

    // 3. Test POST /api/clips
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    let clip_body = r#"{"content":"Hello from phone!","device_label":"Pixel 8"}"#;
    let clip_req = format!(
        "POST /api/clips HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        clip_body.len(), clip_body
    );
    stream.write_all(clip_req.as_bytes()).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("Hello from phone!"));

    // 4. Test GET /api/clips
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream
        .write_all(b"GET /api/clips HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("Pixel 8"));

    // 5. Test GET /api/files/download?path=document.pdf
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream
        .write_all(b"GET /api/files/download?path=document.pdf HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("sample pdf content"));

    // 6. Test DELETE /api/files?path=document.pdf
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream
        .write_all(b"DELETE /api/files?path=document.pdf HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    assert!(resp_str.contains("200 OK"));
    assert!(!temp_shared.join("document.pdf").exists());

    // Stop server
    let _ = tx.send(());
    let _ = server_handle.await;
    let _ = fs::remove_dir_all(&temp_shared);
}
