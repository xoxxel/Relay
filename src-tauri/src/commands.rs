use crate::server;
use crate::state::{AppState, ServerStatus};
use tauri::State;
use tokio::sync::oneshot;

fn build_status(inner: &crate::state::ServerInner) -> ServerStatus {
    let lan_url = format!("http://{}:{}", inner.lan_ip, inner.port);
    ServerStatus {
        running: inner.running,
        port: inner.port,
        lan_ip: inner.lan_ip.clone(),
        lan_url,
        mdns_hostname: inner.mdns_hostname.clone(),
        shared_folder: inner.shared_folder.to_string_lossy().to_string(),
        connected_clients: inner.connected_clients,
    }
}

#[tauri::command]
pub async fn get_server_status(state: State<'_, AppState>) -> Result<ServerStatus, String> {
    let mut inner = state.inner.lock().await;
    // Update local IP if not yet detected properly
    if inner.lan_ip == "127.0.0.1" {
        if let Ok(ip) = local_ip_address::local_ip() {
            inner.lan_ip = ip.to_string();
        }
    }
    Ok(build_status(&inner))
}

#[tauri::command]
pub async fn start_server(
    port: Option<u16>,
    state: State<'_, AppState>,
) -> Result<ServerStatus, String> {
    let mut inner = state.inner.lock().await;

    // If server is already running, return current status
    if inner.running {
        return Ok(build_status(&inner));
    }

    let target_port = port.unwrap_or(inner.port);
    let lan_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let app_state_clone = state.inner();

    // Spawn the axum server on Tokio background task
    let server_state = app_state_clone.clone();
    tokio::spawn(async move {
        if let Err(e) = server::run_server(server_state, target_port, shutdown_rx).await {
            eprintln!("Server error: {}", e);
        }
    });

    inner.running = true;
    inner.port = target_port;
    inner.lan_ip = lan_ip;
    inner.shutdown_tx = Some(shutdown_tx);

    println!(
        "Server started at http://{}:{}",
        inner.lan_ip, inner.port
    );

    Ok(build_status(&inner))
}

#[tauri::command]
pub async fn stop_server(state: State<'_, AppState>) -> Result<ServerStatus, String> {
    let mut inner = state.inner.lock().await;

    if inner.running {
        if let Some(tx) = inner.shutdown_tx.take() {
            let _ = tx.send(());
        }
        inner.running = false;
        println!("Server stopped");
    }

    Ok(build_status(&inner))
}

