use serde::{Serialize, Deserialize};
use ts_rs::TS;
use tauri::State;

use crate::AppState;
use crate::utilities::{self, gen_rand_id, get_curr_time};
use crate::message::{Message, MessageData};

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
            self.name.clone(), 
            self.uid, 
            gen_rand_id(), 
            get_curr_time(),
            self.pic.clone()
        ))
    }
}

#[tauri::command]
pub fn cmd_personalize_new_profile(
    new_name: &str,
    new_pic: &str, 
    state: State<AppState>,
) -> Profile {
    // Update the profile with the profile options the user is allowed
    // to select.

    let mut profile = state.profile.lock().unwrap();
        
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

    state.connection.set_active(true);

    profile.clone()
}