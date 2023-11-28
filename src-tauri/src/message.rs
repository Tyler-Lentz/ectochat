use std::sync::{Arc, Mutex};
use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub enum Message {
    Text(MessageData), 
    Image(MessageData),
    Hello(MessageData),
    Ack{mid: u64, uid: Option<u64>},
}

#[derive(TS, Serialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct MessageData {
    name: String,
    uid: u64,
    mid: u64,
    timestamp: u64,
    payload: Vec<u8>,
}

impl MessageData {
    pub fn new(name: String, uid: u64, mid: u64, timestamp: u64, payload: Vec<u8>) -> MessageData {
        MessageData { name, uid, mid, timestamp, payload }
    }
}

pub struct MessageHistory {
    msgs: Arc<Mutex<Vec<Message>>>,
}

impl MessageHistory {
    pub fn new() -> MessageHistory {
        MessageHistory {msgs: Arc::new(Mutex::new(Vec::new()))}
    }
}
