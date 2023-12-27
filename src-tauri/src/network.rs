use std::{net::{UdpSocket, TcpStream, IpAddr, SocketAddr, TcpListener, Ipv4Addr}, sync::{Mutex, Arc}, time::Duration, collections::HashSet, io::{Write, Read}};
use tauri::{State, async_runtime, Manager};
use const_format::formatcp;
use crate::{message::{Message, MessageData, HEADER_LEN}, utilities::{gen_rand_id, get_curr_time, send_msg_to_frontend, parse_img_str}, profile::Profile};
use crate::utilities;
use crate::AppState;

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
    pub peer_addr: SocketAddr,
    pub local_addr: SocketAddr,
}

impl PeerConnection {
    fn new(stream: TcpStream, stream_type: TcpStreamType) -> Self {
        let peer_addr = stream.peer_addr().unwrap();
        let local_addr = stream.local_addr().unwrap();

        log::info!("Successfully made tcp stream to {}", peer_addr.ip());
        Self { stream, stream_type, peer_profile: None, peer_addr, local_addr }
    }
}

fn is_localhost_stream(stream: &TcpStream) -> bool {
    stream.peer_addr().unwrap().ip() == stream.local_addr().unwrap().ip()
}

pub struct ConnectionState {
    broadcast_socket: Arc<Mutex<UdpSocket>>,
    p2p_connections: Arc<Mutex<Vec<PeerConnection>>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_listeners: Arc<Mutex<Vec<TcpListener>>>,

    active: Arc<Mutex<bool>>,
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
            active: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_active(&self, val: bool) {
        *self.active.lock().unwrap() = val;
    }
}

pub fn run_background_threads(window: tauri::Window) {
    let state: State<AppState> = window.state();

    let active = state.connection.active.clone();
    let w1 = window.clone();
    async_runtime::spawn(async move {
        loop {
            if *active.lock().unwrap() {
                manage_p2p_connections(&w1);
                listen_for_p2p_connections(&w1);
            }
            tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
        }
    });

    let active = state.connection.active.clone();
    let w2 = window.clone();
    async_runtime::spawn(async move {
        loop {
            if *active.lock().unwrap() {
                send_broadcast(&w2);
                listen_for_broadcasts(&w2);
            }
            tokio::time::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME)).await;
        }
    });
}

fn manage_p2p_connections(window: &tauri::Window) {
    let state: State<AppState> = window.state();

    let mut outgoing_acks: Vec<Message> = vec![];
    let mut killed_connections: HashSet<SocketAddr> = HashSet::new();

    {
        let mut p2p_connections = state.connection.p2p_connections.lock().unwrap();
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
                        log::trace!("Only {num_bytes_read} on the wire, so not enough for header. Skipping.");
                        continue
                    }

                    let msg_len = u64::from_le_bytes(buf); // len of msg object
                    let full_msg_len = HEADER_LEN + msg_len as usize; // include 8 bytes from header
                    let mut full_msg_buf = vec![0u8; full_msg_len];
                    match connection.stream.peek(&mut full_msg_buf) {
                        Ok(num_bytes_read) => {
                            if num_bytes_read < full_msg_len {
                                log::trace!("Only {num_bytes_read} bytes on the wire, when expecting {full_msg_len}. Skipping.");
                                continue
                            }

                            // Ok... so this is where we have been trying to get this
                            // whole time. Now we have the entire msg in the full_msg_buf
                            // from 0..full_msg_len
                            let rec_msg = Message::from_network(&full_msg_buf[0..full_msg_len]);

                            // pull out the bytes we used from the buffer
                            let _ = connection.stream.read_exact(&mut full_msg_buf);

                            log::info!("Received {} byte {} message from {}", msg_len, rec_msg.get_type_str(), connection.peer_addr);

                            // add to msg history
                            {
                                let mut msg_history = state.msg_history.lock().unwrap();
                                msg_history.push(rec_msg.clone());
                            }

                            // Record profile if it is a new connection established
                            {
                                match &rec_msg {
                                    Message::Hello(data) => {
                                    // If this is a greeting from a new peer/user, we need to record their
                                    // information so we can poll it later
                                        let mut known_users = state.known_users.lock().unwrap();
                                        let rec_profile = Profile {
                                            name: data.name.clone(),
                                            uid: data.uid, 
                                            join_time: data.timestamp, 
                                            pic: data.payload.clone(),
                                        };
                                        log::info!("Adding {} to known users.", rec_profile.name);
                                        known_users.add_user(rec_profile.clone(), window);

                                        // also add profile information to the connection
                                        connection.peer_profile = Some(rec_profile);
                                    },
                                    Message::Goodbye(_) => {
                                        // This peer is going to be shutting down soon, so we should
                                        // clean up their connection status
                                        log::info!("Goodbye received from {}", connection.peer_addr);
                                        killed_connections.insert(connection.peer_addr);
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
                                            state.profile.lock().unwrap().uid
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
                            log::error!("Error peeking tcp stream for full msg: {e:#?}");
                            continue
                        }
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // no data, so don't wait around for it to come
                    continue
                },
                Err(e) => {
                    log::error!("Error peeking tcp stream for header: {e:#?}");
                    continue
                }
            }
        }

        // Get rid of connections we received a Goodbye message for
        p2p_connections.retain(|conn| {
            !killed_connections.contains(&conn.peer_addr)
        });

        // Remove from set of IPs we're talking to
        let mut p2p_ips = state.connection.p2p_ips.lock().unwrap();
        for peer_addr in killed_connections {
            log::trace!("Removing ips from set: {}", peer_addr.ip());
            p2p_ips.remove(&peer_addr.ip());
        }
    }

    send_msgs_to_all_peers(outgoing_acks, window);
}

fn send_broadcast(window: &tauri::Window) {
    let state: State<AppState> = window.state();

    let msg = Message::Broadcast(state.profile.lock().unwrap().uid).to_network();

    match state.connection.broadcast_socket.lock().unwrap().send_to(&msg, BROADCAST_ADDR) {
        Ok(bytes_written) => {
            if bytes_written != msg.len() {
                log::error!("Error: could not send entire broadcast message");
            }
        },
        Err(e) => {
            log::error!("Error sending broadcast: {e:#?}");
        },
    };
}

fn listen_for_p2p_connections(window: &tauri::Window) {
    let state: State<AppState> = window.state();

    let p2p_listeners = state.connection.p2p_listeners.lock().unwrap();
    for listener in p2p_listeners.iter() {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = stream.set_nonblocking(true);
                    {
                        let mut p2p_ips = state.connection.p2p_ips.lock().unwrap();
                        let peer_ip = stream.peer_addr().unwrap().ip();
                        // keep track that we have an active connection with this ip
                        p2p_ips.insert(peer_ip); 
                    }

                    let stream_type = if is_localhost_stream(&stream) {
                        TcpStreamType::Read
                    } else {
                        let profile = state.profile.lock().unwrap();
                        // Send initial hello msg
                        if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                            log::error!("Error writing hello msg to listen stream: {e:#?}");
                        }

                        TcpStreamType::Both
                    };

                    {
                        let mut p2p_streams = state.connection.p2p_connections.lock().unwrap();
                        // add stream so we start doing listening on it
                        p2p_streams.push(PeerConnection::new(stream, stream_type)); 
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                },
                Err(e) => log::error!("{e}"),
            }
        }
    }
}

fn listen_for_broadcasts(window: &tauri::Window) {
    let state: State<AppState> = window.state();

    let bcast_socket = state.connection.broadcast_socket.lock().unwrap();
    let mut buf = [0; 100]; // broadcast msgs will be tiny 
    match bcast_socket.recv_from(&mut buf) {
        Ok((_received, rec_saddr)) => {
            let rec_msg = Message::from_network(&buf);
            match &rec_msg {
                Message::Broadcast(rec_uid) => {
                    let profile = state.profile.lock().unwrap();
                    if *rec_uid > profile.uid {
                        log::trace!(
                            "Received broadcast from uid={}. Their uid is greater, so waiting for them to establish a connection.",
                            *rec_uid
                        );
                        return 
                    }
                },
                _ => {
                    log::warn!("Received non-broadcast msg on the udp socket: {buf:#?}");
                    return;
                }
            }

            let ip = rec_saddr.ip();
            
            let mut p2p_ips = state.connection.p2p_ips.lock().unwrap();
            if p2p_ips.contains(&ip) {
                log::trace!("Already received broadcast from {ip}, so ignoring");
            } else {
                log::trace!("New broadcast from {ip}, so attempting to establish connection.");
                p2p_ips.insert(ip);

                let possible_tcp_saddrs: Vec<SocketAddr> = (MIN_P2P_PORT..=MAX_P2P_PORT).map(|port| {
                    SocketAddr::new(ip, port)
                }).collect();
                match TcpStream::connect(&possible_tcp_saddrs[..]) {
                    Ok(mut stream) => {
                        let _ = stream.set_nonblocking(true);
                        {
                            let profile = state.profile.lock().unwrap(); 
                            if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                                log::error!("Error writing hello msg to connect stream: {e:#?}");
                            }
                        }
                        {
                            let stream_type = if is_localhost_stream(&stream) {
                                // We have made the stream with ourselves, so now we can tell the frontend
                                // to start displaying the chatting screen
                                let _ = window.emit("evt_start_chatting", "");

                                TcpStreamType::Write
                            } else {
                                TcpStreamType::Both
                            };

                            let mut p2p_connections = state.connection.p2p_connections.lock().unwrap();
                            p2p_connections.push(PeerConnection::new(stream, stream_type));
                        }
                    },
                    Err(err) => {
                        log::error!("Error establishing connection with {ip}, so removing IP from set. {err}");
                        p2p_ips.remove(&ip);
                    },
                }
            }
        },
        _ => (),
    }
}

pub fn send_msgs_to_all_peers(msgs: Vec<Message>, window: &tauri::Window) {
    let state: State<AppState> = window.state();

    state.connection.p2p_connections.lock().unwrap().retain_mut(|connection| {
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

                    state.connection.p2p_ips.lock().unwrap().remove(&connection.peer_addr.ip());

                    send_msg_to_frontend(&dropped_msg, window);
                    state.msg_history.lock().unwrap().push(dropped_msg);

                    log::warn!("Stream at {} no longer valid. Manufacturing drop message.", connection.peer_addr.ip());
                }
                return false; // remove from list, so connection will be dropped
            }
        }

        true
    });
}

#[tauri::command]
pub fn cmd_send_text(msg: &str, state: State<AppState>, window: tauri::Window) {
    let (name, uid) = {
        let profile = state.profile.lock().unwrap();
        (profile.name.clone(), profile.uid)
    };

    let msg = Message::Text(MessageData::new(
        name,
        uid,
        gen_rand_id(),
        get_curr_time(),
        msg.as_bytes().to_vec()
    ));

    send_msgs_to_all_peers(vec![msg], &window);
}

#[tauri::command]
pub fn cmd_send_img(img: &str, state: State<AppState>, window: tauri::Window) {
    let (name, uid) = {
        let profile = state.profile.lock().unwrap();
        (profile.name.clone(), profile.uid)
    };

    let msg = Message::Image(MessageData::new(
        name,
        uid,
        gen_rand_id(),
        get_curr_time(),
        parse_img_str(img),
    ));

    send_msgs_to_all_peers(vec![msg], &window);
}