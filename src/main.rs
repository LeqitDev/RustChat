use core::time;
use std::net::{UdpSocket};
use std::io::{Write, self, ErrorKind};
use std::str::from_utf8;
use std::thread;
use std::sync::mpsc::{self, Receiver};

use msg_types::Type;
use rand::Rng;

mod msg_types;

fn main() {
    let (sender, receiver) = mpsc::channel::<String>();

    // start client on second thread
    let client = thread::spawn(move || {
        udp_client(receiver);
    });

    // start cmd application
    println!("Welcome to RustChat!");

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");

        let tokens: Vec<&str> = input.trim().split(" ").collect();
        let command = tokens[0];
        let args = &tokens[1..];

        match command {
            "stop" => {
                // disconnect from server and stop client thread
                sender.send("stop".to_string()).unwrap();
                println!("Disconnecting...");
                client.join().unwrap();
                break;
            },
            "send" => {
                // tell the client thread to send args to server
                sender.send(format!("send!::!{}", args.join(" "))).unwrap();
            }
            _ => println!("Unknown command")
        }
    }
}

fn udp_client(receiver: Receiver<String>) {
    let mut rng = rand::thread_rng();
    let port = rng.gen_range(1025..65535);
    let udp_sock = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap(); // start client socket on random port
    udp_sock.connect("127.0.0.1:9191").unwrap(); // try to connect to server
    udp_sock.set_nonblocking(true).expect("Failed to enter non-blocking mode"); // dont block the loop
    let mut connected = false;
    write(&udp_sock, "", Type::Connect); // connect on the server

    loop {
        // for the following code see https://github.com/LeqitDev/RustChat/blob/server/src/chat_server.rs#L23
        let mut buffer = vec![0; 4096];
        match udp_sock.recv(&mut buffer) {
            Ok(len) => {
                if len > 8 {
                    let raw_msg = &buffer[..len];
                    if is_msg_valid(raw_msg) {
                        let (msg_type, msg) = deconstruct_msg(raw_msg);
                        match msg_type {
                            Type::ConnectSuccess => connected = true, // client is connected
                            Type::Broadcast => println!("!!BROADCAST!!: {}", msg), // broadcast msg from the server print it TODO: do something
                            _ => (),
                        }
                    }
                }
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => eprintln!("Error on receiving data: {}", e),
        }

        match receiver.recv_timeout(time::Duration::from_millis(10)) {
            Ok(msg) => {
                if !msg.is_empty() {
                    let tokens: Vec<&str> = msg.trim().split("!::!").collect();
                    let command = tokens[0];
                    let args = &tokens[1..];
                    match command {
                        "send" => {
                            // look if the client is connected to the server
                            if connected {
                                // connected: send the message
                                println!("Sending: {}", args[0]);
                                write(&udp_sock, args[0], Type::Broadcast);
                            } else {
                                println!("Not connected to server");
                            }
                        },
                        "stop" => {
                            // send disconnecting message to server
                            write(&udp_sock, "", Type::Disconnect);
                            udp_sock.set_nonblocking(false).unwrap(); // Wait for server response
                            match udp_sock.recv(&mut buffer) {
                                Ok(len) => {
                                    if len == 9 {
                                        let raw_msg = &buffer[..len];
                                        if is_msg_valid(raw_msg) {
                                            let (msg_type, _msg) = deconstruct_msg(raw_msg);
                                            match msg_type {
                                                Type::DisconnectSuccess => println!("Disconnected!"), // Server disconnected the client properly
                                                _ => println!("Forcing disconnect!"),
                                            }
                                        } else {
                                            println!("Forcing disconnect!");
                                        }
                                    } else {
                                        println!("Forcing disconnect!");
                                    }
                                },
                                Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
                                Err(e) => eprintln!("Error on receiving data: {}", e),
                            }
                            break;
                        },
                        _ => {
                            println!("Failed to execute the message");
                        }
                    }
                }
            },
            Err(_) => (),
        }
    }
}

// check https://github.com/LeqitDev/RustChat/blob/server/src/server_utils.rs#L18
fn create_checksum_str(msg: &str) -> u64 {
    return crc64::crc64(1892763397649723641, msg.as_bytes());
}

fn create_checksum(msg: &[u8]) -> u64 {
    return crc64::crc64(1892763397649723641, msg);
}

// check https://github.com/LeqitDev/RustChat/blob/server/src/chat_server.rs#L64
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

// check https://github.com/LeqitDev/RustChat/blob/server/src/server_utils.rs#L5
fn write(sock: &UdpSocket, msg_str: &str, msg_type: Type) {
    let msg_type: u8 = msg_type as u8;
    let msg_checksum: u64 = create_checksum_str(msg_str);
    let mut msg: Vec<u8> = Vec::new();

    msg.push(msg_type);
    msg.extend_from_slice(&msg_checksum.to_ne_bytes());
    msg.extend_from_slice(msg_str.as_bytes());

    sock.send(&msg).unwrap();
}


/* fn get_time_now_in_sec() -> Option<u64> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => return Some(time.as_secs()),
        Err(e) => {
            eprintln!("Something went wrong fetching the time: {}", e);
            return None;
        },
    }
} */