use std::{net::UdpSocket, sync::{Mutex, Arc}, thread, time::Duration, };
use serde::Serialize;
use tauri::State;
use crate::message::{Message, MessageData};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_ADDR: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";

const BUF_SIZE: usize = 1000;

pub struct ConnectionState {
    socket: Arc<Mutex<UdpSocket>>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_nonblocking(true).unwrap();
        socket.connect("0.0.0.0:0").unwrap();


        ConnectionState {
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn start_listen(&self) {
        let socket = self.socket.clone();

        thread::spawn(move || {
            loop {
                {
                    let socket = socket.lock().unwrap();
                    let mut buf = [0; BUF_SIZE];
                    match socket.recv(&mut buf) {
                        Ok(received) => println!("received {received} bytes: {:#?}", &buf[..received]),
                        _ => (),
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

#[tauri::command]
pub fn cmd_send_hello(conn: State<ConnectionState>) {
    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(b"Hello World!", format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}

#[tauri::command]
pub fn cmd_send_text(msg: &str, conn: State<ConnectionState>, profile: State<ProfileState>) {
    let profile = profile.mtx.lock().unwrap();

    let msg = Message::Text(MessageData::new(
        profile.name.clone(),
        profile.uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        msg.as_bytes().to_vec(),
    ));

    let msg = serde_json::to_string(&msg).unwrap();

    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(msg.as_bytes(), format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}