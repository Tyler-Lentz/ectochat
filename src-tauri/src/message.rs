use std::time::Duration;
use std::sync::{Arc, Mutex};

pub enum Message {
    Text(MessageData), 
    Image(MessageData),
}

pub struct MessageData {
    name: String,
    uid: u64,
    timestamp: Duration,
    payload: Box<[u8]>,
}

impl MessageData {
    pub fn new(name: String, uid: u64, timestamp: Duration, payload: Box<[u8]>) -> MessageData {
        MessageData { name, uid, timestamp, payload }
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
