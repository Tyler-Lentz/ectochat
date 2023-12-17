use std::{net::{UdpSocket, TcpStream, IpAddr, SocketAddr, TcpListener, Ipv4Addr}, sync::{Mutex, Arc}, time::Duration, collections::HashSet, io::{Write, Read}};
use tauri::{State, async_runtime};
use const_format::formatcp;
use crate::{message::{Message, MessageData, HEADER_LEN, MessageHistory}, utilities::{KnownUsers, gen_rand_id, get_curr_time, send_msg_to_frontend}, profile::Profile};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_IP: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";
const BROADCAST_ADDR: &str = formatcp!("{BROADCAST_IP}:{BROADCAST_PORT}");
const MIN_P2P_PORT: u16 = 61000;
const MAX_P2P_PORT: u16 = 61255;

const SLEEP_TIME: u64 = 100; // wait 100ms between tcp listener code
const BROADCAST_SLEEP_TIME: u64 = 200; // wait 200ms between broadcast code

#[derive(PartialEq)]
pub enum TcpStreamType {
    Read,
    Write,
    Both,
}

pub struct PeerConnection {
    pub stream: TcpStream,
    pub stream_type: TcpStreamType,
    pub peer_profile: Option<Profile>, // set later once hello msg received
}

impl PeerConnection {
    fn new(stream: TcpStream, stream_type: TcpStreamType) -> Self {
        Self { stream, stream_type, peer_profile: None }
    }
}

fn is_localhost_stream(stream: &TcpStream) -> bool {
    // TODO: remove unsafe unwrap calls
    stream.peer_addr().unwrap().ip() == stream.local_addr().unwrap().ip()
}

pub struct ConnectionState {
    pub broadcast_socket: Arc<Mutex<UdpSocket>>,
    pub p2p_connections: Arc<Mutex<Vec<PeerConnection>>>,
    pub p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    pub p2p_listeners: Arc<Mutex<Vec<TcpListener>>>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        let socket = UdpSocket::bind(format!("0.0.0.0:{BROADCAST_PORT}")).unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_nonblocking(true).unwrap();

        let listeners = (MIN_P2P_PORT..=MAX_P2P_PORT)
            .filter_map(|port| {
                let res = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), port));
                if let Ok(listener) = &res {
                    let _ = listener.set_nonblocking(true);
                }
                res.ok()
            })
            .collect();

        ConnectionState {
            broadcast_socket: Arc::new(Mutex::new(socket)),
            p2p_connections: Arc::new(Mutex::new(Vec::new())),
            p2p_ips: Arc::new(Mutex::new(HashSet::new())),
            p2p_listeners: Arc::new(Mutex::new(listeners)),
        }
    }

    pub fn manage_connections(
        &self,
        window: tauri::Window,
        profile: Arc<Mutex<Profile>>,
        msg_history: Arc<Mutex<Vec<Message>>>,
        known_users: Arc<Mutex<KnownUsers>>,
    ) {
        {
            let p2p_connections = self.p2p_connections.clone();
            let profile = profile.clone();
            let window = window.clone();
            let msg_history = msg_history.clone();
            let known_users = known_users.clone();

            async_runtime::spawn(async move {
                loop {
                    manage_p2p_connections(
                        window.clone(),
                        msg_history.clone(), 
                        p2p_connections.clone(), 
                        known_users.clone(),
                        profile.clone(),
                    );
                    tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
                }
            });
        }

        {
            let broadcast_socket = self.broadcast_socket.clone();
            let profile = profile.clone();

            async_runtime::spawn(async move {
                loop {
                    send_broadcast(profile.clone(), broadcast_socket.clone());
                    tokio::time::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME)).await;
                }
            });
        }

        {
            let profile = profile.clone();
            let broadcast_socket = self.broadcast_socket.clone();
            let p2p_ips = self.p2p_ips.clone();
            let p2p_connections = self.p2p_connections.clone();

            async_runtime::spawn(async move {
                loop {
                    listen_for_broadcasts(
                        profile.clone(), 
                        broadcast_socket.clone(),  
                        p2p_ips.clone(), 
                        p2p_connections.clone()
                    );
                    tokio::time::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME)).await;
                }
            });
        }

        {
            let profile = profile.clone();
            let p2p_listeners = self.p2p_listeners.clone();
            let p2p_ips = self.p2p_ips.clone();
            let p2p_connections = self.p2p_connections.clone();

            async_runtime::spawn(async move {
                loop {
                    listen_for_p2p_connections(
                        profile.clone(), 
                        p2p_listeners.clone(), 
                        p2p_ips.clone(), 
                        p2p_connections.clone()
                    );
                    tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
                }
            });
        }
    }
}

fn manage_p2p_connections(
    window: tauri::Window,
    msg_history: Arc<Mutex<Vec<Message>>>,
    p2p_connections: Arc<Mutex<Vec<PeerConnection>>>,
    known_users: Arc<Mutex<KnownUsers>>,
    profile: Arc<Mutex<Profile>>,
) {
    let mut outgoing_acks: Vec<Message> = vec![];
    let mut killed_connections: Vec<u32> = vec![]; // kill connections with this uid
    // TODO: change killed connections to do it off of the IP in the peer connection instead of uid

    {
        let mut p2p_connections = p2p_connections.lock().unwrap();
        for connection in p2p_connections.iter_mut() {
            if connection.stream_type == TcpStreamType::Write {
                // If this stream should only be used for writing, skip
                // because this function handles listening
                continue
            }

            let mut buf = [0u8; HEADER_LEN];
            match connection.stream.peek(&mut buf) {
                Ok(num_bytes_read) => {
                    if num_bytes_read < HEADER_LEN{
                        // not enough bytes on the wire for entire header, so stop
                        continue
                    }

                    let msg_len = u64::from_le_bytes(buf); // len of msg object
                    let full_msg_len = HEADER_LEN + msg_len as usize; // include 8 bytes from header
                    let mut full_msg_buf = vec![0u8; full_msg_len];
                    match connection.stream.peek(&mut full_msg_buf) {
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
                            let _ = connection.stream.read_exact(&mut full_msg_buf);

                            println!("Received {} byte {} message from {}", msg_len, rec_msg.get_type_str(), connection.stream.peer_addr().unwrap());

                            // add to msg history
                            {
                                let mut msg_history = msg_history.lock().unwrap();
                                msg_history.push(rec_msg.clone());
                            }

                            // Record profile if it is a new connection established
                            {
                                match &rec_msg {
                                    Message::Hello(data) => {
                                    // If this is a greeting from a new peer/user, we need to record their
                                    // information so we can poll it later
                                        let mut known_users = known_users.lock().unwrap();
                                        let rec_profile = Profile {
                                            name: data.name.clone(),
                                            uid: data.uid, 
                                            join_time: data.timestamp, 
                                            pic: data.payload.clone(),
                                        };
                                        known_users.uid_to_profile.insert(data.uid, rec_profile.clone());

                                        // also add profile information to the connection
                                        connection.peer_profile = Some(rec_profile);
                                    },
                                    Message::Goodbye(data) => {
                                        // This peer is going to be shutting down soon, so we should
                                        // clean up their connection status
                                        killed_connections.push(data.uid);
                                    },
                                    _ => {},
                                }
                            }

                            // Send ack msg back
                            {
                                match &rec_msg {
                                    Message::Image(data) |
                                    Message::Text(data)  => {
                                        // If from self, don't ACK
                                        let uid = {
                                            profile.lock().unwrap().uid
                                        };

                                        if data.uid != uid {
                                            // Send back Ack
                                            let ack_msg = Message::Ack{
                                                uid: uid,
                                                mid: data.mid,
                                            };

                                            outgoing_acks.push(ack_msg);
                                        }
                                    },
                                    // don't care about hello or broadcast msg, 
                                    // also, more importantly, don't want to ack acks
                                    // because that would create an infinite loop of
                                    // packets bouncing across the network 
                                    _ => {} 
                                }
                            }

                            send_msg_to_frontend(&rec_msg, &window);
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

        for uid in killed_connections {
            p2p_connections.remove()
            let p2p_connections = p2p_connections
                .iter()
                .filter(|conn| conn.peer_profile.is_some_and(|prof| prof.uid == uid))
                .collect();
        }
    }

    send_msgs_to_all_peers(outgoing_acks, p2p_connections.clone(), msg_history.clone(), &window);

}

fn send_broadcast(
    profile: Arc<Mutex<Profile>>,
    broadcast_socket: Arc<Mutex<UdpSocket>>,
) {
    let msg = Message::Broadcast(profile.lock().unwrap().uid).to_network();

    match broadcast_socket.lock().unwrap().send_to(&msg, BROADCAST_ADDR) {
        Ok(bytes_written) => {
            if bytes_written != msg.len() {
                println!("Error: could not send entire broadcast message");
            }
        },
        Err(e) => {
            println!("Error sending broadcast: {e:#?}");
        },
    }
}

fn listen_for_p2p_connections(
    profile: Arc<Mutex<Profile>>,
    p2p_listeners: Arc<Mutex<Vec<TcpListener>>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_connections: Arc<Mutex<Vec<PeerConnection>>>,
) {
    let p2p_listeners = p2p_listeners.lock().unwrap();
    for listener in p2p_listeners.iter() {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = stream.set_nonblocking(true);
                    {
                        let mut p2p_ips = p2p_ips.lock().unwrap();
                        let peer_ip = stream.peer_addr().unwrap().ip();
                        // keep track that we have an active connection with this ip
                        p2p_ips.insert(peer_ip); 
                    }

                    let stream_type = if is_localhost_stream(&stream) {
                        TcpStreamType::Read
                    } else {
                        let profile = profile.lock().unwrap();
                        // Send initial hello msg
                        if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                            println!("Error writing hello msg to listen stream: {e:#?}");
                        }

                        TcpStreamType::Both
                    };

                    {
                        let mut p2p_streams = p2p_connections.lock().unwrap();
                        // add stream so we start doing listening on it
                        p2p_streams.push(PeerConnection::new(stream, stream_type)); 
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                },
                Err(e) => println!("{e}"),
            }
        }
    }
}

fn listen_for_broadcasts(
    profile: Arc<Mutex<Profile>>,
    broadcast_socket: Arc<Mutex<UdpSocket>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_connections: Arc<Mutex<Vec<PeerConnection>>>,
) {
    let bcast_socket = broadcast_socket.lock().unwrap();
    let mut buf = [0; 100]; // broadcast msgs will be tiny 
    match bcast_socket.recv_from(&mut buf) {
        Ok((_received, rec_saddr)) => {
            let rec_msg = Message::from_network(&buf);
            match &rec_msg {
                Message::Broadcast(rec_uid) => {
                    let profile = profile.lock().unwrap();
                    if *rec_uid > profile.uid {
                        // Their UID is larger, so listen for their TCP connection
                        // Don't need to do anything here, stream will be established
                        // in listen_to_p2p_connections
                        return 
                    }
                },
                _ => {
                    return;
                }
            }

            let ip = rec_saddr.ip();
            
            let mut p2p_ips = p2p_ips.lock().unwrap();
            if p2p_ips.contains(&ip) {
                // do nothing
            } else {
                p2p_ips.insert(ip);

                let possible_tcp_saddrs: Vec<SocketAddr> = (MIN_P2P_PORT..=MAX_P2P_PORT).map(|port| {
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
                            let stream_type = if is_localhost_stream(&stream) {
                                TcpStreamType::Write
                            } else {
                                TcpStreamType::Both
                            };

                            let mut p2p_connections = p2p_connections.lock().unwrap();
                            p2p_connections.push(PeerConnection::new(stream, stream_type));
                        }
                        println!("Successfully made tcp stream to {ip}");
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

pub fn send_msgs_to_all_peers(
    msgs: Vec<Message>, 
    connections: Arc<Mutex<Vec<PeerConnection>>>,
    msg_history: Arc<Mutex<Vec<Message>>>,
    window: &tauri::Window,
) {
    connections.lock().unwrap().retain_mut(|connection| {
        if connection.stream_type == TcpStreamType::Read {
            return true; // keep but don't do anything
        }

        for msg in &msgs {
            let msg_network = &msg.to_network();
            let expected_bytes = msg_network.len();

            let stream_valid = match connection.stream.write(msg_network) {
                Ok(bytes_written) => bytes_written == expected_bytes,
                Err(_) => false,
            };

            if !stream_valid {
                if let Some(profile) = &connection.peer_profile {
                    // if we know who the connection was from, then manufacture a dropped msg
                    // and insert into the frontend so that it can display the connection was dropped
                    let dropped_msg = Message::Dropped(MessageData::new(
                        profile.name.clone(),
                        profile.uid,
                        gen_rand_id(),
                        get_curr_time(),
                        profile.pic.clone(),
                    ));

                    send_msg_to_frontend(&dropped_msg, window);
                    msg_history.lock().unwrap().push(dropped_msg);
                }
                return false; // remove from list, so connection will be dropped
            }
        }

        true
    });
}

#[tauri::command]
pub fn cmd_send_text(
    msg: &str,
    conn: State<ConnectionState>,
    profile: State<ProfileState>,
    msg_history: State<MessageHistory>,
    window: tauri::Window,
) {
    let (name, uid) = {
        let profile = profile.profile.lock().unwrap();
        (profile.name.clone(), profile.uid)
    };

    let msg = Message::Text(MessageData::new(
        name,
        uid,
        utilities::gen_rand_id(),
        utilities::get_curr_time(),
        msg.as_bytes().to_vec()
    ));

    send_msgs_to_all_peers(vec![msg], conn.p2p_connections.clone(), msg_history.msgs.clone(), &window);
}