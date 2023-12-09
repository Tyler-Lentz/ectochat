use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use tauri::State;

use crate::network::ConnectionState;
use crate::utilities::{self, KnownUsersState, gen_rand_id, get_curr_time};
use crate::message::{MessageHistory, Message, MessageData};

#[derive(TS, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct Profile {
    pub name: String,
    pub uid: u32,
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

    pub fn make_hello_msg(&self) -> Message {
        Message::Hello(MessageData::new(
            self.name, 
            self.uid, 
            gen_rand_id(), 
            get_curr_time(),
            b"Hello World!".to_vec(), 
            self.pic
        ))
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
    profile_state: State<ProfileState>, 
    conn: State<ConnectionState>,
    msg_history: State<MessageHistory>,
    known_users: State<KnownUsersState>,
    window: tauri::Window,
) -> Profile {
    // Update the profile with the profile options the user is allowed
    // to select.

    let mut profile = profile_state.profile.lock().unwrap();
        
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

    conn.start_listen(window, profile.uid, msg_history, known_users);

    profile.clone()
}

pub struct ProfileState {
    pub profile: Arc<Mutex<Profile>>,
}

impl ProfileState {
    pub fn new() -> ProfileState {
        ProfileState {
            profile: Arc::new(Mutex::new(Profile::new(generate_random_name())))
        }
    }
}