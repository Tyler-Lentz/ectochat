use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use ts_rs::TS;

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

#[tauri::command]
pub fn cmd_create_profile(name: &str) -> Profile {
    Profile::new(String::from(name))
}