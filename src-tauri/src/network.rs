use std::{net::UdpSocket, sync::{Mutex, Arc}, thread, time::Duration, mem};
use tauri::State;

const BROADCAST_ADDR: &str = "255.255.255.255";
const LISTEN_ADDR: &str = "0.0.0.0";

const BUF_SIZE: usize = 10000;

pub struct ConnectionState {
    socket: Arc<Mutex<UdpSocket>>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        let socket = UdpSocket::bind(format!("{LISTEN_ADDR}:0")).unwrap();
        socket.set_broadcast(true).unwrap();


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
                        Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
                        Err(e) => println!("recv function failed: {e:?}"),
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

#[tauri::command]
pub fn cmd_send_hello(conn: State<ConnectionState>) {

}