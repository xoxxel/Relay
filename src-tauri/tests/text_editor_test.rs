use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use relay_lib::{
    server::create_router,
    state::{AppState, ServerInner},
};
use std::{fs, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower::ServiceExt;

#[tokio::test]
async fn text_routes_read_save_conflict_and_reject_unsafe_files() {
    let base = std::env::temp_dir().join(format!("relay-api-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&base).unwrap();
    fs::write(base.join("note.txt"), "hello").unwrap();
    fs::write(base.join("binary.dat"), [0, 255, 1]).unwrap();
    let (broadcast_tx, _) = broadcast::channel(16);
    let state = AppState {
        inner: Arc::new(Mutex::new(ServerInner {
            running: false,
            port: 0,
            shared_folder: base.clone(),
            lan_ip: "127.0.0.1".into(),
            mdns_hostname: "relay.local".into(),
            device_name: "Test".into(),
            connected_clients: 0,
            shutdown_tx: None,
            mdns: None,
        })),
        db: Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap())),
        broadcast_tx,
    };
    let router = create_router(state);
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/files/text?path=note.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let data = to_bytes(response.into_body(), 2048).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&data).unwrap()["content"],
        "hello"
    );
    for expected in [StatusCode::NO_CONTENT, StatusCode::CONFLICT] {
        let body =
            serde_json::json!({"path":"note.txt", "original":"hello", "content":"سلام edited"})
                .to_string();
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/files/text")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
    }
    assert_eq!(
        fs::read_to_string(base.join("note.txt")).unwrap(),
        "سلام edited"
    );
    for (path, expected) in [
        ("binary.dat", StatusCode::UNSUPPORTED_MEDIA_TYPE),
        ("missing", StatusCode::NOT_FOUND),
        ("%2E%2E%2Foutside", StatusCode::FORBIDDEN),
    ] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/files/text?path={path}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
    }
    fs::remove_dir_all(base).unwrap();
}
