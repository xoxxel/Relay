use crate::mdns::MdnsBroadcaster;
use crate::server;
use crate::state::{AppState, ServerStatus, Settings};
use std::path::PathBuf;
use tauri::State;
use tokio::sync::oneshot;

fn build_status(inner: &crate::state::ServerInner) -> ServerStatus {
    let lan_url = crate::net::format_url(&inner.lan_ip, inner.port);
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
    if inner.lan_ip == "127.0.0.1" {
        if let Some(ip) = crate::net::detect_lan_ip() {
            inner.lan_ip = ip;
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

    if inner.running {
        return Ok(build_status(&inner));
    }

    let target_port = port.unwrap_or(inner.port);
    let lan_ip = crate::net::detect_lan_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let app_state_clone = state.inner();

    let server_state = app_state_clone.clone();
    tokio::spawn(async move {
        if let Err(e) = server::run_server(server_state, target_port, shutdown_rx).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Start mDNS broadcasting
    let mdns = MdnsBroadcaster::start(target_port, &inner.mdns_hostname).ok();

    inner.running = true;
    inner.port = target_port;
    inner.lan_ip = lan_ip;
    inner.shutdown_tx = Some(shutdown_tx);
    inner.mdns = mdns;

    println!("Server started at http://{}:{}", inner.lan_ip, inner.port);

    Ok(build_status(&inner))
}

#[tauri::command]
pub async fn stop_server(state: State<'_, AppState>) -> Result<ServerStatus, String> {
    let mut inner = state.inner.lock().await;

    if inner.running {
        if let Some(tx) = inner.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(mdns) = inner.mdns.take() {
            mdns.stop();
        }
        inner.running = false;
        println!("Server stopped");
    }

    Ok(build_status(&inner))
}

#[tauri::command]
pub async fn pick_shared_folder(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let dialog = rfd::AsyncFileDialog::new().set_title("Select Shared Folder");

    if let Some(folder) = dialog.pick_folder().await {
        let path = folder.path().to_path_buf();
        let path_str = path.to_string_lossy().to_string();

        let mut inner = state.inner.lock().await;
        inner.shared_folder = path;

        Ok(Some(path_str))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn copy_to_system_clipboard(text: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| format!("Failed to access system clipboard: {}", e))?;
        clipboard
            .set_text(text)
            .map_err(|e| format!("Failed to write to system clipboard: {}", e))?;
        Ok(())
    })
    .await
    .map_err(|e| format!("Clipboard task failed: {}", e))?
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let inner = state.inner.lock().await;
    Ok(Settings {
        port: inner.port,
        shared_folder: inner.shared_folder.to_string_lossy().to_string(),
        device_name: inner.device_name.clone(),
    })
}

#[tauri::command]
pub async fn save_settings(
    settings: Settings,
    state: State<'_, AppState>,
) -> Result<Settings, String> {
    let mut inner = state.inner.lock().await;
    inner.port = settings.port;
    inner.shared_folder = PathBuf::from(&settings.shared_folder);
    inner.device_name = settings.device_name.clone();

    Ok(settings)
}
