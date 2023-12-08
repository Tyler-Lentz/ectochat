use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

#[derive(TS, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub enum Message {
    Text(MessageData), 
    Image(MessageData),
    Hello(MessageData),
    Ack{mid: u32, uid: Option<u32>},
}

impl Message {
    pub fn compress(&self) -> Vec<u8> {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        if let Err(err) = e.write_all(serde_json::to_string(&self).unwrap().as_bytes()) {
            println!("{err}");
        }
        e.finish().unwrap()
    }

    pub fn from_compressed(buf: &[u8]) -> Self {
        let mut d = GzDecoder::new(buf);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        serde_json::from_str(&s).unwrap()
    }
}

#[derive(TS, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct MessageData {
    pub name: String,
    pub uid: u32,
    pub mid: u32,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub pic: Vec<u8>,
}

impl MessageData {
    pub fn new(name: String, uid: u32, mid: u32, timestamp: u64, payload: Vec<u8>, pic: Vec<u8>) -> MessageData {
        MessageData { name, uid, mid, timestamp, payload, pic }
    }
}

pub struct MessageHistory {
    pub msgs: Arc<Mutex<Vec<Message>>>,

    /*
        Hashset of all the ids received
        an entry of (mid, uid) means that 
        mid has already been received from uid.

        Text/Image/Hello msgs are literally uid send mid
        Acks are 1:1 with how the uids/mids are sent in the ack
     */
    pub ids: Arc<Mutex<HashSet<(u32, u32)>>>,
}

impl MessageHistory {
    pub fn new() -> MessageHistory {
        MessageHistory {
            msgs: Arc::new(Mutex::new(Vec::new())),
            ids: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}