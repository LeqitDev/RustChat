#[derive(Debug)]
pub enum Type {
    Connect,
    SendSuccess,
    SendFailed,
    Broadcast,
    SendTo,
    ConnectSuccess,
    Disconnect,
}

impl From<u8> for Type {
    fn from(n: u8) -> Self {
        match n {
            0 => Type::Connect,
            1 => Type::SendSuccess,
            2 => Type::SendFailed,
            3 => Type::Broadcast,
            4 => Type::SendTo,
            5 => Type::ConnectSuccess,
            6 => Type::Disconnect,
            _ => panic!("Invalid value for MsgType: {}", n),
        }
    }
}