pub mod commands;
pub mod server;
pub mod state;
pub mod web_assets;

use state::AppState;

pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_server_status,
            commands::start_server,
            commands::stop_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

