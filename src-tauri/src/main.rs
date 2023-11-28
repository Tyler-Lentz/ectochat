// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod profile;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![profile::cmd_create_profile])
        .manage(message::MessageHistory::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
