// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use message::{Message, MessageHistory, MessageData};
use profile::ProfileState;
use network::ConnectionState;
use utilities::{KnownUsersState, gen_rand_id, get_curr_time};
use tauri::{Manager, State};

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
        .manage(MessageHistory::new())
        .manage(ProfileState::new())
        .manage(ConnectionState::new())
        .manage(KnownUsersState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::Destroyed => {
            let connection_state: State<ConnectionState> = event.window().state();
            let connections = connection_state.p2p_connections.clone();
            let p2p_ips = connection_state.p2p_ips.clone();

            let msg_history_state: State<MessageHistory> = event.window().state();
            let msg_history = msg_history_state.msgs.clone();

            let profile_state: State<ProfileState> = event.window().state();
            let profile = profile_state.profile.clone();
            let profile = profile.lock().unwrap();

            let goodbye_msg = Message::Goodbye(MessageData::new(
                profile.name.clone(),
                profile.uid,
                gen_rand_id(),
                get_curr_time(),
                profile.pic.clone(),
            ));

            network::send_msgs_to_all_peers(vec![goodbye_msg], connections, msg_history, event.window(), p2p_ips);
        },
        _ => {},
    }
}