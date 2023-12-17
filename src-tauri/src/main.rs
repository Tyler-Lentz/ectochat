// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use message::{Message, MessageData};
use profile::Profile;
use network::ConnectionState;
use utilities::{gen_rand_id, get_curr_time, KnownUsers};
use tauri::{Manager, State};

mod message;
mod profile;
mod network;
mod utilities;

pub struct AppState {
    pub msg_history: Arc<Mutex<Vec<Message>>>,
    pub profile: Arc<Mutex<Profile>>,

    pub known_users: Arc<Mutex<KnownUsers>>,

    pub connection: ConnectionState,
}

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info, 
        simplelog::Config::default(), 
        simplelog::TerminalMode::Mixed, 
        simplelog::ColorChoice::Auto, 
    ).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            profile::cmd_personalize_new_profile,
            network::cmd_send_text,
            utilities::cmd_get_known_users,
        ])
        .on_window_event(handle_window_event)
        .manage(AppState {
            msg_history: Arc::new(Mutex::new(Vec::new())),
            profile: Arc::new(Mutex::new(Profile::new("unnamed".to_owned()))),
            known_users: Arc::new(Mutex::new(KnownUsers::new())),
            connection: ConnectionState::new(),
        })
        .on_page_load(|window, _payload| {
            network::run_background_threads(window);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::Destroyed => {
            let state: State<AppState> = event.window().state();
            let profile = state.profile.lock().unwrap();

            let goodbye_msg = Message::Goodbye(MessageData::new(
                profile.name.clone(),
                profile.uid,
                gen_rand_id(),
                get_curr_time(),
                profile.pic.clone(),
            ));

            network::send_msgs_to_all_peers(vec![goodbye_msg], event.window());
        },
        _ => {},
    }
}