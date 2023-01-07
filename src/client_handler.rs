use std::net::{SocketAddr, UdpSocket};

use crate::{msg_types::Type, server_utils};

#[derive(Debug)]
pub struct Client {
    pub addr: SocketAddr,
    pub client_id: u32,
    // TODO: client name/tag like CubeCoder#1234?
}

impl Client {
    pub fn handle_msg(&self, msg_type: Type, msg: &str, conns: &Vec<Client>, sock: &UdpSocket) {
        match msg_type {
            Type::SendTo => {
                // TODO: handle send to encrypted message (see notion)
                println!("Message from {}: {}", self.addr, msg);
                print!("> ");
                server_utils::write(sock, "", Type::SendSuccess, self.addr);
            },
            Type::Broadcast => {
                // write to all connected clients
                for conn in conns {
                    server_utils::write(sock, msg, Type::Broadcast, conn.addr);
                }
            },
            _ => (),
        }
    }
}