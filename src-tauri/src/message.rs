use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

pub const PADDED_HEADER_LEN: usize = 200;

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub enum Message {
    // Broadcast messages sent to find other hosts
    // Send out UID in broadcast message
    // It is the responsibility of the host with greater 
    // UID to initiate the TCP connection
    Broadcast(MessageHeader),

    // Message sent in response to broadcast, over tcp,
    // to establish TCP connection
    Hello(MessageHeader, Vec<u8>),

    // Messages sent p2p over tcp streams, actual data
    // sent via chat
    Text(MessageHeader, Vec<u8>), 
    Image(MessageHeader, Vec<u8>),
    Ack(MessageHeader),
}

impl Message {
    pub fn to_network(&self) -> Vec<u8> {
        let (mut header, payload) = match self {
            Self::Broadcast(header)       |
            Self::Ack(header)             => (header.clone(), None),
            Self::Hello(header, payload)  |
            Self::Text(header, payload)   |
            Self::Image(header, payload)  => (header.clone(), Some(payload)),
        };

        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        if let Err(err) = e.write_all(serde_json::to_string(&self).unwrap().as_bytes()) {
            println!("{err}");
        }
        let payload_bytes = e.finish().unwrap();

        header.payload_len = payload_bytes.len();
        const SPACE_ASCII: u8 = 32;
        let mut header_bytes = serde_json::to_vec(&header).unwrap();
        header_bytes.resize(PADDED_HEADER_LEN, SPACE_ASCII);
        
        [header_bytes, payload_bytes].concat()
    }
}

#[derive(TS, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
#[ts(export)]
#[ts(export_to="../src/lib/bindings/")]
pub struct MessageHeader {
    pub name: String,
    pub uid: u32,
    pub mid: u32,
    pub timestamp: u64,
    pub payload_len: usize,
}

impl MessageHeader {
    // payload_len set to 0, because it will be set to the correct compressed size
    // in the to_network function on Message
    pub fn new(name: String, uid: u32, mid: u32, timestamp: u64 ) -> MessageHeader {
        MessageHeader { name, uid, mid, timestamp, payload_len: 0 }
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