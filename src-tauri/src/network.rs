use std::{net::UdpSocket, sync::{Mutex, Arc}, thread, time::Duration};
use tauri::State;
use crate::message::{Message, MessageData};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_ADDR: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";

pub struct ConnectionState {
    socket: Arc<Mutex<UdpSocket>>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        let socket = UdpSocket::bind(format!("0.0.0.0:{BROADCAST_PORT}")).unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_nonblocking(true).unwrap();

        ConnectionState {
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn start_listen(&self, window: tauri::Window) {
        let socket = self.socket.clone();

        thread::spawn(move || {
            loop {
                {
                    let socket = socket.lock().unwrap();
                    let mut buf = [0; u16::MAX as usize];
                    match socket.recv_from(&mut buf) {
                        Ok((received, addr)) => {
                            println!("Received {received} bytes from {addr}");  
                            // TODO handle these errors!! VERY IMPORTANT!
                            // TODO add to message history
                            let msg = Message::from_compressed(&buf[0..received]);
                            let _ = window.emit("evt_new_msg", msg);
                        },
                        _ => (),
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

#[tauri::command]
pub fn cmd_send_hello(conn: State<ConnectionState>, profile: State<ProfileState>) {
    let profile = profile.mtx.lock().unwrap();

    let hello_msg = Message::Hello(MessageData::new(
        profile.name.clone(),
        profile.uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        "Hello World!".as_bytes().to_vec(),
        profile.pic.clone(),
    ));

    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(&hello_msg.compress()[..], format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}

#[tauri::command]
pub fn cmd_send_text(
    msg: &str,
    conn: State<ConnectionState>,
    profile: State<ProfileState>,
) {
    let profile = profile.mtx.lock().unwrap();

    let msg = Message::Text(MessageData::new(
        profile.name.clone(),
        profile.uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        msg.as_bytes().to_vec(),
        profile.pic.clone(),
    ));

    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(&msg.compress()[..], format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}