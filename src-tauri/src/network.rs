use std::{net::UdpSocket, sync::{Mutex, Arc}, thread, time::Duration, ffi::CStr, };
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

    pub fn start_listen(&self, window: tauri::Window) {
        let socket = self.socket.clone();

        thread::spawn(move || {
            loop {
                {
                    let socket = socket.lock().unwrap();
                    let mut buf = [0; BUF_SIZE];
                    match socket.recv(&mut buf) {
                        Ok(received) => {
                            println!("Received {received} bytes");  
                            // TODO handle these errors!! VERY IMPORTANT!
                            let buf = CStr::from_bytes_until_nul(&buf).unwrap().to_bytes();
                            let msg: Message = serde_json::from_slice(buf).unwrap();
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
pub fn cmd_send_hello(conn: State<ConnectionState>) {
    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(b"Hello World!", format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}

#[tauri::command]
pub fn cmd_send_text(
    msg: &str,
    conn: State<ConnectionState>,
    profile: State<ProfileState>,
    window: tauri::Window
) {
    let profile = profile.mtx.lock().unwrap();

    let msg = Message::Text(MessageData::new(
        profile.name.clone(),
        profile.uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        msg.as_bytes().to_vec(),
    ));

    let msg_json = serde_json::to_string(&msg).unwrap();

    let socket = conn.socket.lock().unwrap();
    socket
        .send_to(msg_json.as_bytes(), format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");

    let _ = window.emit("evt_new_msg", msg);
}