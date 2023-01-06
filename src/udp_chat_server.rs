use std::{sync::{atomic::AtomicBool, Arc}, net::{UdpSocket, SocketAddr}, str::from_utf8, io::ErrorKind};
use crate::msg_types::Type;

// Packet: (u8 checksum | u64 checksum) | msg

pub fn start_server(stop: Arc<AtomicBool>) {
    let udp_sock = UdpSocket::bind("127.0.0.1:9191").unwrap();
    udp_sock.set_nonblocking(true).expect("Failed to enter non-blocking mode");
    let mut connections: Vec<SocketAddr> = Vec::new();

    loop {
        if stop.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        let mut buffer = vec![0; 4096];
        match udp_sock.recv_from(&mut buffer) {
            Ok((len , addr)) => {
                if len > 8 {
                    let raw_msg = &buffer[..len];
                    if is_msg_valid(raw_msg) {
                        let (msg_type, msg) = deconstruct_msg(raw_msg);
                        match msg_type {
                            Type::Connect => {
                                connections.push(addr);
                                write(&udp_sock, "", Type::ConnectSuccess, addr);
                            }, // add addr toconnected clients
                            Type::Disconnect => {
                                let index = connections.iter().position(|x| *x == addr).unwrap();
                                connections.swap_remove(index);
                            }, // remove addr from connected clients
                            Type::SendTo => {
                                println!("Message from {}: {}", addr, msg);
                                print!("> ");
                                write(&udp_sock, "", Type::SendSuccess, addr);
                            },
                            Type::Broadcast => {
                                for conn in &connections {
                                    write(&udp_sock, msg, Type::Broadcast, *conn);
                                }
                            },
                            _ => (),
                        }
                    }
                }
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => eprintln!("Error on receiving data: {}", e),
        }
    }
}

fn create_checksum_str(msg: &str) -> u64 {
    return crc64::crc64(1892763397649723641, msg.as_bytes());
}

fn create_checksum(msg: &[u8]) -> u64 {
    return crc64::crc64(1892763397649723641, msg);
}

fn is_msg_valid(msg: &[u8]) -> bool {
    let msg_checksum = u64::from_ne_bytes(msg[1..9].try_into().unwrap());
    let msg = &msg[9..msg.len()];
    msg_checksum == create_checksum(msg)
}

fn deconstruct_msg(msg: &[u8]) -> (Type, &str) {
    let msg_type: Type = msg[0].into();
    let msg = &msg[9..msg.len()];
    let msg_str = from_utf8(msg).unwrap();
    (msg_type, msg_str)
}

fn write(sock: &UdpSocket, msg_str: &str, msg_type: Type, addr: SocketAddr) {
    let msg_type: u8 = msg_type as u8;
    let msg_checksum: u64 = create_checksum_str(msg_str);
    let mut msg: Vec<u8> = Vec::new();

    msg.push(msg_type);
    msg.extend_from_slice(&msg_checksum.to_ne_bytes());
    msg.extend_from_slice(msg_str.as_bytes());

    sock.send_to(&msg, addr).unwrap();
}