use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap, sync::{Mutex, Arc}};
use tauri::State;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::profile::Profile;
use crate::message::Message;

pub fn gen_rand_id() -> u32 {
    rand::random()
}

pub fn get_curr_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}

#[derive(TS, Serialize, Deserialize, Clone)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct KnownUsers {
    pub uid_to_profile: HashMap<u32, Profile>,
}

impl KnownUsers {
    pub fn new() -> Self {
        KnownUsers {
            uid_to_profile: HashMap::new()
        }
    }
}

pub struct KnownUsersState {
    // Map that stores uids and associates with usernames
    pub users: Arc<Mutex<KnownUsers>>,
}

impl KnownUsersState {
    pub fn new() -> Self {
        KnownUsersState { users: Arc::new(Mutex::new(KnownUsers::new())) }
    }
}

#[tauri::command]
pub fn cmd_get_known_users(known_users: State<KnownUsersState>) -> KnownUsers {
    let map = known_users.users.lock().unwrap();
    map.clone()
}

pub fn send_msg_to_frontend(msg: &Message, window: &tauri::Window) {
    let res = window.emit("evt_new_msg", msg);
    if let Err(e) = res {
        println!("evt_new_msg err {e:#?}");
    }
}