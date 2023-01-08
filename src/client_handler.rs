use std::{net::{SocketAddr, UdpSocket}, str::from_utf8};

use crate::{msg_types::Type, server_utils};

#[derive(Debug)]
pub struct Client {
    pub addr: SocketAddr,
    pub client_id: u32,
    // TODO: client name/tag like CubeCoder#1234?
}

impl Client {
    pub fn handle_msg(&self, msg_type: Type, msg: &[u8], conns: &Vec<Client>, sock: &UdpSocket) {
        match msg_type {
            Type::SendTo => {
                // TODO: handle send to encrypted message (see notion)
                println!("Message from {}: {}", self.addr, u8_to_str(msg));
                print!("> ");
                server_utils::write(sock, "", Type::SendSuccess, self.addr);
            },
            Type::Broadcast => {
                // write to all connected clients
                for conn in conns {
                    server_utils::write(sock, u8_to_str(msg), Type::Broadcast, conn.addr);
                }
            },
            _ => (),
        }
    }
}

fn u8_to_str(buf: &[u8]) -> &str {
    from_utf8(buf).unwrap()
}