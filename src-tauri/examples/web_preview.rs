//! Isolated browser review: cargo run --example web_preview -- /tmp/relay-preview 19421
//! Uses a separate database and binds only to loopback; never opens the user's Relay database.
use relay_lib::{
    server::create_router,
    state::{AppState, ServerInner},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{broadcast, Mutex};
#[tokio::main]
async fn main() {
    let folder = PathBuf::from(std::env::args().nth(1).expect("provide a scratch folder"));
    std::fs::create_dir_all(&folder).unwrap();
    let port: u16 = std::env::args()
        .nth(2)
        .unwrap_or("19421".into())
        .parse()
        .unwrap();
    let db = relay_lib::db::init_db(&folder.join(".preview.sqlite")).unwrap();
    let (broadcast_tx, _) = broadcast::channel(64);
    let state = AppState {
        inner: Arc::new(Mutex::new(ServerInner {
            running: true,
            port,
            shared_folder: folder,
            lan_ip: "127.0.0.1".into(),
            mdns_hostname: "relay.local".into(),
            device_name: "Preview".into(),
            connected_clients: 0,
            shutdown_tx: None,
            mdns: None,
        })),
        db: Arc::new(Mutex::new(db)),
        broadcast_tx,
    };
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .unwrap();
    println!("Isolated Relay preview: http://127.0.0.1:{port}");
    axum::serve(listener, create_router(state)).await.unwrap();
}
