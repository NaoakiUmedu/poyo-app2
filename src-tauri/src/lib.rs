mod controller;
mod repository;

// Dialy
use crate::controller::dialy_controller::{delete_command, insert_command, select_all_command};
use crate::repository::dialy_repository::DialyRepository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DialyRepository::new("dialies.db".to_string()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            insert_command,
            delete_command,
            select_all_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
