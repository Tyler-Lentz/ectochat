use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use tauri::State;

use crate::network::ConnectionState;
use crate::utilities;

#[derive(TS, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct Profile {
    pub name: String,
    pub uid: u64,
    pub join_time: u64,
    pub pic: Vec<u8>,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        Profile {
            name,
            uid: utilities::gen_rand_id(),
            join_time: utilities::get_curr_time(),
            pic: Vec::new()
        }
    }
}

fn generate_random_name() -> String {
    // TODO: generate silly random name
    // mayhap: https://crates.io/crates/random_name_generator
    return String::from("unnamed");
} 

#[tauri::command]
pub fn cmd_personalize_new_profile(
    new_name: &str,
    new_pic: &str, 
    profile: State<ProfileState>, 
    conn: State<ConnectionState>,
    window: tauri::Window,
) -> Profile {
    // Update the profile with the profile options the user is allowed
    // to select.

    let mut profile = profile.mtx.lock().unwrap();
        
    if new_name != "" {
        profile.name = String::from(new_name);
    }

    // pic being sent as comma-separated string, so convert back into array
    profile.pic = new_pic
        .split(',')
        .map(|char| {
            char.parse::<u8>().unwrap()
        })
        .collect();

    conn.start_listen(window, profile.uid);

    profile.clone()
}

pub struct ProfileState {
    pub mtx: Mutex<Profile>,
}

impl ProfileState {
    pub fn new() -> ProfileState {
        ProfileState {mtx: Mutex::new(Profile::new(generate_random_name()))}
    }
}