use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

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
pub struct ServerStatus {
    pub running: bool,
    pub port: u16,
    pub lan_ip: String,
    pub lan_url: String,
    pub mdns_hostname: String,
    pub shared_folder: String,
    pub connected_clients: usize,
}

pub struct ServerInner {
    pub running: bool,
    pub port: u16,
    pub shared_folder: PathBuf,
    pub lan_ip: String,
    pub mdns_hostname: String,
    pub connected_clients: usize,
    pub shutdown_tx: Option<oneshot::Sender<()>>,
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Mutex<ServerInner>>,
}

impl AppState {
    pub fn new() -> Self {
        let default_dir = std::env::var("HOME")
            .map(|h| PathBuf::from(h).join("Shared"))
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        Self {
            inner: Arc::new(Mutex::new(ServerInner {
                running: false,
                port: 8420,
                shared_folder: default_dir,
                lan_ip: "127.0.0.1".to_string(),
                mdns_hostname: "relay.local".to_string(),
                connected_clients: 0,
                shutdown_tx: None,
            })),
        }
    }
}

