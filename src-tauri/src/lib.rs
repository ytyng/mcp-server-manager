mod claude_config;
mod commands;

use commands::{get_mcp_servers, get_projects, set_project_disabled_servers, set_server_enabled};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            get_mcp_servers,
            set_server_enabled,
            get_projects,
            set_project_disabled_servers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
