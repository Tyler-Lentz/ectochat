// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod profile;
mod network;
mod utilities;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            profile::cmd_personalize_new_profile,
            network::cmd_send_hello,
            network::cmd_send_text,
            utilities::cmd_get_known_users,
        ])
        .manage(message::MessageHistory::new())
        .manage(profile::ProfileState::new())
        .manage(network::ConnectionState::new())
        .manage(utilities::KnownUsersState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
