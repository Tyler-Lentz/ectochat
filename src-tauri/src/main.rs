// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod profile;
mod network;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            profile::cmd_set_profile_name,
            network::cmd_send_hello,
        ])
        .manage(message::MessageHistory::new())
        .manage(profile::ProfileState::new())
        .manage(network::ConnectionState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
