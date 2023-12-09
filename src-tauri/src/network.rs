use std::{net::{UdpSocket, TcpStream, IpAddr, SocketAddr, TcpListener}, sync::{Mutex, Arc}, thread, time::Duration, collections::HashSet, io::{Write, Read}};
use tauri::State;
use crate::{message::{Message, MessageData, HEADER_LEN}, utilities::{KnownUsersState, gen_rand_id, get_curr_time}, profile::Profile};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_ADDR: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";
const MIN_P2P_PORT: u16 = 59813;
const MAX_P2P_PORT: u16 = 60000; // allow like 200ish p2p connections, don't expect much more...

const SLEEP_TIME: u64 = 100; // wait 100ms between checking for new messages from peers
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
        let _ = listener.set_nonblocking(true);

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
        window: tauri::Window,
        profile: Arc<Mutex<Profile>>,
        msg_history: Arc<Mutex<Vec<Message>>>,
    ) {
        self.manage_p2p_connections(window, msg_history);
        self.send_broadcasts(profile.clone());
        self.listen_for_p2p_connections(profile.clone());
        self.listen_for_broadcasts(profile);
    }

    fn manage_p2p_connections(
        &self,
        window: tauri::Window,
        msg_history: Arc<Mutex<Vec<Message>>>,
    ) {
        let p2p_streams = self.p2p_streams.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME));
                {
                    let p2p_streams = p2p_streams.lock().unwrap();
                    for mut stream in p2p_streams.iter() {
                        let mut buf = [0u8; HEADER_LEN];
                        match stream.peek(&mut buf) {
                            Ok(num_bytes_read) => {
                                if num_bytes_read < HEADER_LEN{
                                    // not enough bytes on the wire for entire header, so stop
                                    continue
                                }

                                let msg_len = u64::from_le_bytes(buf); // len of msg object
                                let full_msg_len = HEADER_LEN + msg_len as usize; // include 8 bytes from header
                                let mut full_msg_buf = vec![0u8; full_msg_len];
                                match stream.peek(&mut full_msg_buf) {
                                    Ok(num_bytes_read) => {
                                        if num_bytes_read < full_msg_len {
                                            // Full message is not there yet
                                            continue
                                        }

                                        // Ok... so this is where we have been trying to get this
                                        // whole time. Now we have the entire msg in the full_msg_buf
                                        // from 0..full_msg_len
                                        let rec_msg = Message::from_network(&full_msg_buf[0..full_msg_len]);

                                        // pull out the bytes we used from the buffer
                                        let _ = stream.read_exact(&mut full_msg_buf);

                                        println!("Received {} byte message from {}", msg_len, stream.peer_addr().unwrap());

                                        // add to msg history
                                        {
                                            let mut msg_history = msg_history.lock().unwrap();
                                            msg_history.push(rec_msg.clone());
                                        }

                                        // send msg to frontend
                                        let res = window.emit("evt_new_msg", rec_msg);
                                        if let Err(e) = res {
                                            println!("evt_new_msg err {e:#?}");
                                        }
                                    },
                                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                        continue
                                    },
                                    Err(e) => {
                                        println!("Error peeking tcp stream for full msg: {e:#?}");
                                        continue
                                    }
                                }
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // no data, so don't wait around for it to come
                                continue
                            },
                            Err(e) => {
                                println!("Error peeking tcp stream for header: {e:#?}");
                                continue
                            }
                        }
                    }
                }
            }
        });
    }

    fn send_broadcasts(
        &self,
        profile: Arc<Mutex<Profile>>
    ) {
        let uid = {
            profile.lock().unwrap().uid
        };

        let socket = self.broadcast_socket.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME));
                {
                    socket.lock().unwrap()
                        .send_to(&Message::Broadcast(uid).to_network(), format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
                        .expect("Couldn't send msg");
                }
            }
        });
    }

    fn listen_for_p2p_connections(
        &self, 
        profile: Arc<Mutex<Profile>>
    ) {
        let p2p_listener = self.p2p_listener.clone();
        let p2p_ips = self.p2p_ips.clone();
        let p2p_streams = self.p2p_streams.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                {
                    let p2p_listener = p2p_listener.lock().unwrap();
                    for stream in p2p_listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                let _ = stream.set_nonblocking(true);
                                {
                                    let mut p2p_ips = p2p_ips.lock().unwrap();
                                    let peer_ip = stream.peer_addr().unwrap().ip();
                                    // keep track that we have an active connection with this ip
                                    p2p_ips.insert(peer_ip); 
                                }
                                {
                                    let profile = profile.lock().unwrap();
                                    // Send initial hello msg
                                    if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                                        println!("Error writing hello msg to listen stream: {e:#?}");
                                    }
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

    fn listen_for_broadcasts(
        &self, 
        profile: Arc<Mutex<Profile>>
    ) {
        let bcast_socket = self.broadcast_socket.clone();
        let p2p_ips = self.p2p_ips.clone();
        let p2p_streams = self.p2p_streams.clone();

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
                                        let _ = stream.set_nonblocking(true);
                                        {
                                            let profile = profile.lock().unwrap(); 
                                            if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                                                println!("Error writing hello msg to connect stream: {e:#?}");
                                            }
                                        }
                                        {
                                            let mut p2p_streams = p2p_streams.lock().unwrap();
                                            p2p_streams.push(stream);
                                        }
                                    },
                                    Err(_err) => {
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
pub fn cmd_send_text(
    msg: &str,
    conn: State<ConnectionState>,
    profile: State<ProfileState>,
) {
    let profile = profile.profile.lock().unwrap();

    let msg = Message::Text(MessageData::new(
        profile.name.clone(),
        profile.uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        msg.as_bytes().to_vec()
    ));

    let p2p_streams = conn.p2p_streams.lock().unwrap();

    for mut stream in p2p_streams.iter() {
        if let Err(e) = stream.write(&msg.to_network()) {
            println!("Error writing text msg to {}: {:#?}", stream.peer_addr().unwrap(), e);
        };
    }
}