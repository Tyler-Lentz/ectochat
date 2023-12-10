use std::{net::{UdpSocket, TcpStream, IpAddr, SocketAddr, TcpListener, Ipv4Addr}, sync::{Mutex, Arc}, time::Duration, collections::HashSet, io::{Write, Read}};
use tauri::{State, async_runtime};
use crate::{message::{Message, MessageData, HEADER_LEN}, utilities::KnownUsers, profile::Profile};
use crate::profile::ProfileState;
use crate::utilities;

const BROADCAST_ADDR: &str = "255.255.255.255";
const BROADCAST_PORT: &str = "59813";
const MIN_P2P_PORT: u16 = 61000;
const MAX_P2P_PORT: u16 = 61255;

const SLEEP_TIME: u64 = 100; // wait 100ms between tcp listener code
const BROADCAST_SLEEP_TIME: u64 = 200; // wait 200ms between broadcast code

#[derive(PartialEq)]
enum TcpStreamType {
    Read,
    Write,
    Both,
}

fn is_localhost_stream(stream: &TcpStream) -> bool {
    stream.peer_addr().unwrap().ip() == stream.local_addr().unwrap().ip()
}

pub struct ConnectionState {
    broadcast_socket: Arc<Mutex<UdpSocket>>,
    p2p_streams: Arc<Mutex<Vec<(TcpStreamType, TcpStream)>>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_listeners: Arc<Mutex<Vec<TcpListener>>>,
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
            p2p_streams: Arc::new(Mutex::new(Vec::new())),
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
            let p2p_streams = self.p2p_streams.clone();
            let profile = profile.clone();
            let window = window.clone();
            let msg_history = msg_history.clone();
            let known_users = known_users.clone();

            async_runtime::spawn(async move {
                loop {
                    manage_p2p_connections(
                        window.clone(),
                        msg_history.clone(), 
                        p2p_streams.clone(), 
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
            let p2p_streams = self.p2p_streams.clone();

            async_runtime::spawn(async move {
                loop {
                    listen_for_broadcasts(
                        profile.clone(), 
                        broadcast_socket.clone(),  
                        p2p_ips.clone(), 
                        p2p_streams.clone()
                    );
                    tokio::time::sleep(Duration::from_millis(BROADCAST_SLEEP_TIME)).await;
                }
            });
        }

        {
            let profile = profile.clone();
            let p2p_listeners = self.p2p_listeners.clone();
            let p2p_ips = self.p2p_ips.clone();
            let p2p_streams = self.p2p_streams.clone();

            async_runtime::spawn(async move {
                loop {
                    listen_for_p2p_connections(
                        profile.clone(), 
                        p2p_listeners.clone(), 
                        p2p_ips.clone(), 
                        p2p_streams.clone()
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
    p2p_streams: Arc<Mutex<Vec<(TcpStreamType, TcpStream)>>>,
    known_users: Arc<Mutex<KnownUsers>>,
    profile: Arc<Mutex<Profile>>,
) {
    let mut outgoing_acks: Vec<Message> = vec![];

    let mut p2p_streams = p2p_streams.lock().unwrap();
    for (stream_type, ref mut stream) in p2p_streams.iter_mut() {
        if *stream_type == TcpStreamType::Write {
            // If this stream should only be used for writing, skip
            // because this function handles listening
            continue
        }

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

                        println!("Received {} byte {} message from {}", msg_len, rec_msg.get_type_str(), stream.peer_addr().unwrap());

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
                                    known_users.uid_to_profile.insert(data.uid,
                                        Profile { 
                                            name: data.name.clone(),
                                            uid: data.uid, 
                                            join_time: data.timestamp, 
                                            pic: data.payload.clone(),
                                        }
                                    );
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

    // Second pass, send out ack messages to all relevant streams
    for msg in outgoing_acks {
        for (stream_type, ref mut stream) in p2p_streams.iter_mut() {
            if *stream_type == TcpStreamType::Read {
                // don't send out ack on readonly stream
                continue
            }

            stream.write(&msg.to_network()).expect("Couldn't send ack");
        }
    }
}

fn send_broadcast(
    profile: Arc<Mutex<Profile>>,
    broadcast_socket: Arc<Mutex<UdpSocket>>,
) {
    broadcast_socket.lock().unwrap()
        .send_to(&Message::Broadcast(profile.lock().unwrap().uid).to_network(), format!("{BROADCAST_ADDR}:{BROADCAST_PORT}"))
        .expect("Couldn't send msg");
}

fn listen_for_p2p_connections(
    profile: Arc<Mutex<Profile>>,
    p2p_listeners: Arc<Mutex<Vec<TcpListener>>>,
    p2p_ips: Arc<Mutex<HashSet<IpAddr>>>,
    p2p_streams: Arc<Mutex<Vec<(TcpStreamType, TcpStream)>>>,
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
                        TcpStreamType::Both
                    };

                    if stream_type == TcpStreamType::Both { // only send hello if this isn't localhost
                        let profile = profile.lock().unwrap();
                        // Send initial hello msg
                        if let Err(e) = stream.write(&profile.make_hello_msg().to_network()) {
                            println!("Error writing hello msg to listen stream: {e:#?}");
                        }
                    }
                    {
                        let mut p2p_streams = p2p_streams.lock().unwrap();
                        // add stream so we start doing listening on it
                        p2p_streams.push((stream_type, stream)); 
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
    p2p_streams: Arc<Mutex<Vec<(TcpStreamType, TcpStream)>>>,
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
                            let mut p2p_streams = p2p_streams.lock().unwrap();
                            p2p_streams.push((stream_type, stream));
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

    let mut p2p_streams = conn.p2p_streams.lock().unwrap();

    for (stream_type, ref mut stream) in p2p_streams.iter_mut() {
        if *stream_type == TcpStreamType::Read {
            // if the channel should be only used for read, then dont write to it
            continue
        }

        if let Err(e) = stream.write(&msg.to_network()) {
            println!("Error writing text msg to {}: {:#?}", stream.peer_addr().unwrap(), e);
        };
    }
}