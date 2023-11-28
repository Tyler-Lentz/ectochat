// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;

fn main() {
  tauri::Builder::default()
    .manage(message::MessageHistory::new())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
