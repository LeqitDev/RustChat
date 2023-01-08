use core::time;
use std::{sync::mpsc::Receiver, net::UdpSocket, str::from_utf8, io::ErrorKind, vec};

use crate::{msg_types::Type};
use crate::client_handler::Client;
use crate::server_utils;

// Packet: (u8 checksum | u64 checksum) | msg

pub fn start_server(receiver: Receiver<String>) {
    // Set server ip
    let udp_sock = UdpSocket::bind("127.0.0.1:9191").unwrap();
    // Incoming messages should not block the loop
    udp_sock.set_nonblocking(true).expect("Failed to enter non-blocking mode");
    // store all active connections
    let mut connections: Vec<Client> = Vec::new();
    // log server messages
    let mut server_log = server_utils::Logger {log: vec![], print: false};
    let mut server_mode = false;

    server_log.mode_println("Server started!".to_string(), server_mode);

    loop {
        match receiver.recv_timeout(time::Duration::from_millis(5)) {
            Ok(string) => {
                if !string.is_empty() {
                    match string.as_str() {
                        "stop" => {
                            // If server should stop
                            for conn in connections {
                                server_utils::write(&udp_sock, "", Type::Disconnect, conn.addr);
                            }
                            break;
                        },
                        "serverlogs" => {
                            // show/hide serverlogs
                            server_mode = !server_mode;
                            if server_mode {
                                winconsole::console::clear().unwrap();
                                for line in &server_log.log {
                                    println!("{}", *line);
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            Err(_) => (),
        }

        // Buffer with at least 4096 bytes
        let mut buffer = vec![0; 4096];
        // fetch incoming messages
        match udp_sock.recv_from(&mut buffer) {
            Ok((len , addr)) => {
                // Length of message >= 9 header has the size of 9: 1 type byte, 8 checksum bytes 8 bytes = u64
                if len > 8 {
                    let raw_msg = &buffer[..len]; // get the raw message bytes
                    if is_msg_valid(raw_msg) { // check if msg is valid (checksum equal)
                        let (msg_type, msg) = deconstruct_msg(raw_msg); // get the msg type and the msg itself

                        match msg_type {
                            Type::Connect => { // client wants to connect to server
                                server_log.mode_println(format!("Client {} connected!", addr), server_mode);
                                connections.push(Client {addr, client_id: 123}); // create new client entity with TODO: random client id
                                server_utils::write(&udp_sock, "", Type::ConnectSuccess, addr); // Write to the client that connection has been established
                            }, // add client to active connections
                            Type::Disconnect => {
                                server_log.mode_println(format!("Client {} disconnected!", addr), server_mode);
                                let index = connections.iter().position(|x| x.addr == addr).unwrap(); // find index of client entity with the exact addr
                                connections.swap_remove(index); // fill the hole with the last element in the array
                                server_utils::write(&udp_sock, "", Type::DisconnectSuccess, addr); // write to the client that the disconnection was successful
                            }, // remove client from active connections
                            _ => {
                                // find the client with the exact addr and let him handle the message
                                match connections.iter().find(|x| x.addr == addr) {
                                    Some(client) => client.handle_msg(msg_type, msg, &connections, &udp_sock),
                                    None => (),
                                }
                            }
                        }
                    } else {
                        // write to the client that something is wrong with msg or checksum TODO: other msg type
                        server_utils::write(&udp_sock, "", Type::SendFailed, addr);
                    }
                }
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => eprintln!("Error on receiving data: {}", e),
        }
    }
}

// Check if Checksum is equal (crc64)
fn is_msg_valid(msg: &[u8]) -> bool {
    let msg_checksum = u64::from_ne_bytes(msg[1..9].try_into().unwrap());
    let msg = &msg[9..];
    msg_checksum == server_utils::create_checksum(msg)
}

// split header from message return message type and message string
// TODO: Schould return message as byte bc of encrypted message
fn deconstruct_msg_str(msg: &[u8]) -> (Type, &str) {
    let msg_type: Type = msg[0].into();
    let msg = &msg[9..];
    let msg_str = from_utf8(msg).unwrap();
    (msg_type, msg_str)
}

fn deconstruct_msg(msg: &[u8]) -> (Type, &[u8]) {
    let msg_type: Type = msg[0].into();
    let msg = &msg[9..];
    (msg_type, msg)
}