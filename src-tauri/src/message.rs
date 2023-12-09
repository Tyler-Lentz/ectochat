use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

pub const HEADER_LEN: usize = 8; // number of bytes we store the whole msg len in (little endian)

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub enum Message {
    // Broadcast messages sent to find other hosts
    // Send out UID in broadcast message
    // It is the responsibility of the host with greater 
    // UID to initiate the TCP connection
    Broadcast(u32),

    // Message sent in response to broadcast, over tcp,
    // to establish TCP connection
    Hello(MessageData),

    // Messages sent p2p over tcp streams, actual data
    // sent via chat
    Text(MessageData),
    Image(MessageData),
    Ack{ uid: u32, mid: u32 },
}

impl Message {
    // prepends with 8 bytes (little endian) of size of entire packet
    // so it can be picked out from the tcp stream
    pub fn to_network(&self) -> Vec<u8> {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        if let Err(err) = e.write_all(serde_json::to_string(&self).unwrap().as_bytes()) {
            println!("{err}");
        }
        let message_bytes = e.finish().unwrap();
        
        let message_len = message_bytes.len() as u64;

        [message_len.to_le_bytes().to_vec(), message_bytes].concat()
    }

    // buf should still include the 8 bytes that has the size of the whole
    // msg, because this will strip it off, and it should not go further
    // than the end of the message
    pub fn from_network(buf: &[u8]) -> Self {
        let buf = &buf[HEADER_LEN..]; // strip off 8 byte header
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
}

impl MessageData {
    pub fn new(name: String, uid: u32, mid: u32, timestamp: u64, payload: Vec<u8> ) -> MessageData {
        MessageData { name, uid, mid, timestamp, payload }
    }
}

pub struct MessageHistory {
    pub msgs: Arc<Mutex<Vec<Message>>>,
}

impl MessageHistory {
    pub fn new() -> MessageHistory {
        MessageHistory {
            msgs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}