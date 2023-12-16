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
            network::cmd_send_text,
            utilities::cmd_get_known_users,
        ])
        .on_window_event(handle_window_event)
        .manage(message::MessageHistory::new())
        .manage(profile::ProfileState::new())
        .manage(network::ConnectionState::new())
        .manage(utilities::KnownUsersState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::Destroyed => {
            println!("Window closed");
            // TODO: send Goodbye messages to all peers
        },
        _ => {},
    }
}