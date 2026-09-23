pub mod backup;
pub mod commands;
pub mod hashing;
pub mod models;

use commands::{get_backup_history, inspect_folder, start_local_backup};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            inspect_folder,
            start_local_backup,
            get_backup_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
