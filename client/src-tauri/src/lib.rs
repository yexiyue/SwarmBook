pub mod commands;
use commands::{generate_peer_id, send_ping_command, start_ping_node};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate_peer_id,
            start_ping_node,
            send_ping_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
