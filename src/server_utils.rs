use std::net::{UdpSocket, SocketAddr};

use crate::msg_types::Type;

// assemble message and write to addr
pub fn write(sock: &UdpSocket, msg_str: &str, msg_type: Type, addr: SocketAddr) {
    let msg_type: u8 = msg_type as u8; // msg type as u8
    let msg_checksum: u64 = create_checksum_str(msg_str); // checksum as u8 array
    let mut msg: Vec<u8> = Vec::new(); // message u8 array

    msg.push(msg_type); // add the message type
    msg.extend_from_slice(&msg_checksum.to_ne_bytes()); // add the checksum
    msg.extend_from_slice(msg_str.as_bytes()); // add the real msg

    sock.send_to(&msg, addr).unwrap(); // send to addr
}

pub fn create_checksum_str(msg: &str) -> u64 {
    // TODO: const crc key
    return crc64::crc64(1892763397649723641, msg.as_bytes());
}

pub fn create_checksum(msg: &[u8]) -> u64 {
    return crc64::crc64(1892763397649723641, msg);
}