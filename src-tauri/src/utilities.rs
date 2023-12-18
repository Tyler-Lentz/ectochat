use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap};
use tauri::{State, Manager};
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{profile::Profile, AppState};
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

pub fn send_msg_to_frontend(msg: &Message, window: &tauri::Window) {
    let res = window.emit("evt_new_msg", msg);
    if let Err(e) = res {
        log::error!("evt_new_msg err {e:#?}");
    }
}

#[derive(TS, Serialize, Deserialize, Clone)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct KnownUsers {
    uid_to_profile: HashMap<u32, Profile>,
}

impl KnownUsers {
    pub fn new() -> Self {
        KnownUsers {
            uid_to_profile: HashMap::new()
        }
    }

    pub fn add_user(&mut self, prof: Profile, window: &tauri::Window) {
        self.uid_to_profile.insert(prof.uid, prof);
        window.emit("evt_known_users_changed", self.clone());
    }

    pub fn does_user_exist(&self, uid: u32) -> bool {
        self.uid_to_profile.contains_key(&uid)
    }

    pub fn remove_user(&mut self, uid: u32, window: &tauri::Window) -> Option<Profile> {
        let old_profile = self.uid_to_profile.remove(&uid);
        window.emit("evt_known_users_changed", self.clone());
        old_profile
    }
}

#[tauri::command]
pub fn cmd_get_known_users(state: State<AppState>) -> KnownUsers {
    state.known_users.lock().unwrap().clone()
}
