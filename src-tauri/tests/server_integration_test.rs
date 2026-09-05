use relay_lib::server;
use relay_lib::state::AppState;
use std::fs;
use tokio::sync::oneshot;

#[tokio::test]
async fn test_axum_server_and_web_client_serving() {
    let app_state = AppState::new();
    let temp_shared = std::env::temp_dir().join("lan_share_integration_test_shared");
    let _ = fs::create_dir_all(&temp_shared);
    fs::write(temp_shared.join("testfile.txt"), b"Hello LAN Share").unwrap();

    {
        let mut inner = app_state.inner.lock().await;
        inner.shared_folder = temp_shared.clone();
    }

    let port = 18420;
    let (tx, rx) = oneshot::channel();
    let state_clone = app_state.clone();

    let server_handle = tokio::spawn(async move {
        server::run_server(state_clone, port, rx).await.unwrap();
    });

    // Allow server to bind
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // 1. Test GET /api/files
    let client = reqwest_or_hyper(port).await;
    assert!(client, "API or root response failed");

    // 2. Stop server
    let _ = tx.send(());
    let _ = server_handle.await;
    let _ = fs::remove_dir_all(&temp_shared);
}

async fn reqwest_or_hyper(port: u16) -> bool {
    // Simple TCP / HTTP request
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream
        .write_all(b"GET /api/files HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let resp_str = String::from_utf8_lossy(&response);
    println!("API Response: {}", resp_str);
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("testfile.txt"));

    // Test GET /
    let mut stream2 = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream2
        .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut response2 = Vec::new();
    stream2.read_to_end(&mut response2).await.unwrap();
    let resp_str2 = String::from_utf8_lossy(&response2);
    println!("Web Client Root Response: {}", resp_str2);
    assert!(resp_str2.contains("200 OK"));

    true
}
