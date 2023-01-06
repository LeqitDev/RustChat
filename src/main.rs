use std::thread;
use std::io::{Write, self};
use std::sync::{Arc, atomic::AtomicBool};

mod udp_chat_server;
mod msg_types;


fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_ref = stop.clone();

    let server = thread::spawn(move || {
        udp_chat_server::start_server(stop_ref);
    });

    println!("Welcome to RustChatServerConsole!");

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");

        let tokens: Vec<&str> = input.trim().split(" ").collect();
        let command = tokens[0];
        let _args = &tokens[1..];

        
        match command {
            "stop" => {
                println!("Stopping server...");
                stop.store(true, std::sync::atomic::Ordering::SeqCst);
                server.join().unwrap();
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
}