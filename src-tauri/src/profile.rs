use std::{time::{SystemTime, UNIX_EPOCH}, sync::{Arc, Mutex}};
use serde::Serialize;
use ts_rs::TS;
use tauri::State;

use crate::network::ConnectionState;

#[derive(TS, Serialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct Profile {
    name: String,
    uid: u64,
    join_time: u64,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        Profile {
            name,
            uid: rand::random(),
            join_time: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0,
            },
        }
    }
}

fn generate_random_name() -> String {
    // TODO: generate silly random name
    // mayhap: https://crates.io/crates/random_name_generator
    return String::from("unnamed");
} 

#[tauri::command]
pub fn cmd_set_profile_name(name: &str, profile: State<ProfileState>, conn: State<ConnectionState>) -> Profile {
    let mut profile = profile.mtx.lock().unwrap();
        
    if name != "" {
        profile.name = String::from(name);
    }

    conn.start_listen();

    profile.clone()
}

pub struct ProfileState {
    mtx: Mutex<Profile>,
}

impl ProfileState {
    pub fn new() -> ProfileState {
        ProfileState {mtx: Mutex::new(Profile::new(generate_random_name()))}
    }
}