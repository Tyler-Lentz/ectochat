use std::{net::{UdpSocket, TcpStream, IpAddr, SocketAddr, TcpListener}, sync::{Mutex, Arc}, thread, time::Duration, collections::HashSet, io::{Write, Read}};
use tauri::State;
use crate::{message::{Message, MessageData, MessageHistory}, utilities::KnownUsersState};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_ADDR: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";
const MIN_P2P_PORT: u16 = 59813;
const MAX_P2P_PORT: u16 = 60000; // allow like 200ish p2p connections, don't expect much more...

const SLEEP_TIME: u64 = 100;
const BROADCAST_SLEEP_TIME: u64 = 1000; // wait for 1s between sending out broadcasts

pub struct ConnectionState {
    broadcast_socket: Arc<Mutex<UdpSocket>>,
    p2p_streams: Arc<Mutex<Vec<TcpStream>>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_listener: Arc<Mutex<TcpListener>>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        let socket = UdpSocket::bind(format!("0.0.0.0:{BROADCAST_PORT}")).unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_nonblocking(true).unwrap();

        let listener = TcpListener::bind("0.0.0.0:0").unwrap();
        listener.set_nonblocking(true);

        // TODO: make tcp listener outside so cna set nonblocking
        ConnectionState {
            broadcast_socket: Arc::new(Mutex::new(socket)),
            p2p_streams: Arc::new(Mutex::new(Vec::new())),
            p2p_ips: Arc::new(Mutex::new(HashSet::new())),
            p2p_listener: Arc::new(Mutex::new(listener)),
        }
    }

    pub fn manage_connections(
        &self,
        profile_state: State<ProfileState>
    ) {
        self.manage_p2p_connections();
        self.send_broadcasts(profile_state);
        self.listen_for_p2p_connections(profile_state);
        self.listen_for_broadcasts(profile_state);
    }

    fn manage_p2p_connections(
        &self,
    ) {
        let p2p_streams = self.p2p_streams.clone();

        thread::spawn(|| {
            loop {
                {
                    let p2p_streams = p2p_streams.lock().unwrap();
                    for mut stream in p2p_streams.iter() {
                        let mut buf: Vec<u8> = vec![];
                        stream.read_to_end(&mut buf);
                    }
                }
                thread::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME));
            }
        });
    }

    fn send_broadcasts(
        &self,
        profile_state: State<ProfileState>
    ) {
        let uid = {
            profile_state.profile.lock().unwrap().uid
        };

        thread::spawn(|| {
            let socket = self.broadcast_socket.clone();

            loop {
                thread::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME));
                {
                    let socket = socket.lock().unwrap()
                        .send_to(&Message::Broadcast(uid).to_network(), format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
                        .expect("Couldn't send msg");
                }
            }
        });
    }

    fn listen_for_p2p_connections(&self, profile_state: State<ProfileState>) {
        let p2p_listener = self.p2p_listener.clone();
        let p2p_ips = self.p2p_ips.clone();
        let p2p_streams = self.p2p_streams.clone();
        let profile = profile_state.profile.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                {
                    let p2p_listener = p2p_listener.lock().unwrap();
                    for stream in p2p_listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                stream.set_nonblocking(true);
                                {
                                    let mut p2p_ips = p2p_ips.lock().unwrap();
                                    let peer_ip = stream.peer_addr().unwrap().ip();
                                    // keep track that we have an active connection with this ip
                                    p2p_ips.insert(peer_ip); 
                                }
                                {
                                    let profile = profile.lock().unwrap();
                                    // Send initial hello msg
                                    stream.write(&profile.make_hello_msg().to_network());
                                }
                                {
                                    let mut p2p_streams = p2p_streams.lock().unwrap();
                                    // add stream so we start doing listening on it
                                    p2p_streams.push(stream); 
                                }
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
                            Err(e) => println!("{e}"),
                        }
                    }
                }
            }
        });
    }

    fn listen_for_broadcasts(&self, profile_state: State<ProfileState>) {
        let bcast_socket = self.broadcast_socket.clone();
        let p2p_ips = self.p2p_ips.clone();
        let p2p_streams = self.p2p_streams.clone();
        let profile = profile_state.profile.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                {
                    let bcast_socket = bcast_socket.lock().unwrap();
                    let mut buf = [0; 100]; // broadcast msgs will be tiny 
                    match bcast_socket.recv_from(&mut buf) {
                        Ok((received, rec_saddr)) => {
                            println!("Broadcast: received {received} bytes from {rec_saddr}");  
                            let rec_msg = Message::from_network(&buf);
                            match &rec_msg {
                                Message::Broadcast(rec_uid) => {
                                    let profile = profile.lock().unwrap();
                                    if *rec_uid > profile.uid {
                                        // Their UID is larger, so listen for their TCP connection
                                        // Don't need to do anything here, stream will be established
                                        // in listen_to_p2p_connections
                                        continue
                                    }
                                },
                                _ => {
                                    println!("Non broadcast received over UDP, ignoring");
                                    continue;
                                }
                            }

                            let ip = rec_saddr.ip();
                            
                            let mut p2p_ips = p2p_ips.lock().unwrap();
                            if p2p_ips.contains(&ip) {
                                println!("Already have channel open with {ip}, ignoring broadcast.");
                            } else {
                                p2p_ips.insert(ip);

                                let possible_tcp_saddrs: Vec<SocketAddr> = (MIN_P2P_PORT..MAX_P2P_PORT).map(|port| {
                                    SocketAddr::new(ip, port)
                                }).collect();
                                match TcpStream::connect(&possible_tcp_saddrs[..]) {
                                    Ok(mut stream) => {
                                        stream.set_nonblocking(true);
                                        {
                                            let mut p2p_streams = p2p_streams.lock().unwrap();
                                            p2p_streams.push(stream);
                                        }
                                        {
                                            let profile = profile.lock().unwrap(); 
                                            stream.write(&profile.make_hello_msg().to_network());
                                        }
                                    },
                                    Err(err) => {
                                        p2p_ips.remove(&ip);
                                    },
                                }
                            }
                        },
                        _ => (),
                    }
                }
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

    let socket = conn.broadcast_socket.lock().unwrap();
    for _ in 0..3 {
        socket
            .send_to(&hello_msg.compress()[..], format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
            .expect("Couldn't send msg");
    }
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

    let socket = conn.broadcast_socket.lock().unwrap();

    for _ in 0..3 {
        socket
            .send_to(&msg.compress()[..], format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
            .expect("Couldn't send msg");
    }
}