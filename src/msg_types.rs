#[derive(Debug)]
pub enum Type {
    Connect,
    SendSuccess,
    SendFailed,
    Broadcast,
    SendTo,
    ConnectSuccess,
    Disconnect,
    DisconnectSuccess,
    SendToSuccess,
    SendToFailed,
    EncryptedMessage,
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
            7 => Type::DisconnectSuccess,
            8 => Type::SendToSuccess,
            9 => Type::SendToFailed,
            10 => Type::EncryptedMessage,
            _ => panic!("Invalid value for MsgType: {}", n),
        }
    }
}