use crate::db;
use crate::mdns::MdnsBroadcaster;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, oneshot, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub modified_at: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipItem {
    pub id: String,
    pub content: String,
    pub device_label: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub running: bool,
    pub port: u16,
    pub lan_ip: String,
    pub lan_url: String,
    pub mdns_hostname: String,
    pub shared_folder: String,
    pub connected_clients: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub shared_folder: String,
    pub device_name: String,
}

pub struct ServerInner {
    pub running: bool,
    pub port: u16,
    pub shared_folder: PathBuf,
    pub lan_ip: String,
    pub mdns_hostname: String,
    pub device_name: String,
    pub connected_clients: usize,
    pub shutdown_tx: Option<oneshot::Sender<()>>,
    pub mdns: Option<MdnsBroadcaster>,
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Mutex<ServerInner>>,
    pub db: Arc<Mutex<Connection>>,
    pub broadcast_tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let default_dir = PathBuf::from(&home_dir).join("Shared");
        let db_path = PathBuf::from(&home_dir)
            .join(".local")
            .join("share")
            .join("relay")
            .join("relay.db");

        let conn = db::init_db(&db_path).unwrap_or_else(|_| {
            let fallback_db = PathBuf::from("relay.db");
            db::init_db(&fallback_db).expect("Failed to initialize SQLite database")
        });

        let (broadcast_tx, _) = broadcast::channel(256);

        Self {
            inner: Arc::new(Mutex::new(ServerInner {
                running: false,
                port: 8420,
                shared_folder: default_dir,
                lan_ip: "127.0.0.1".to_string(),
                mdns_hostname: "relay.local".to_string(),
                device_name: "Host PC".to_string(),
                connected_clients: 0,
                shutdown_tx: None,
                mdns: None,
            })),
            db: Arc::new(Mutex::new(conn)),
            broadcast_tx,
        }
    }
}
