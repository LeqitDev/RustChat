use std::sync::mpsc;
// use std::sync::mpsc;
use std::thread;
use std::io::{Write, self};

mod chat_server;
mod msg_types;
mod client_handler;
mod server_utils;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    // run server on second thread
    let server = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(chat_server::start_server(rx));
    });
    /* let server = thread::spawn(move || {
        chat_server::start_server(rx).await;
    }); */

    let mut cmd_mode = true;
    let mut cmd_logs = server_utils::Logger{log: vec![], print: false};

    // start cmd application
    winconsole::console::clear().unwrap();
    cmd_logs.mode_println("Welcome to RustChatServerConsole!".to_string(), cmd_mode);

    loop {
        if cmd_mode {
            print!("> ");
        }
        io::stdout().flush().expect("Error flushing stdout!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");

        let tokens: Vec<&str> = input.trim().split(" ").collect();
        let command = tokens[0];
        let _args = &tokens[1..];

        if cmd_mode {
            cmd_logs.log("> ".to_string());
            cmd_logs.logln(format!("{}", input.replace("\r\n", "")));
            match command {
                "stop" => {
                    // stop server
                    cmd_logs.mode_println("Stopping server...".to_string(), cmd_mode);
                    tx.send("stop".to_string()).unwrap();
                    server.join().unwrap();
                    cmd_logs.mode_println("Exiting...".to_string(), cmd_mode);
                    break;
                },
                "serverlogs" => {
                    tx.send("serverlogs".to_string()).unwrap();
                    cmd_mode = false;
                },
                _ => {
                    cmd_logs.mode_println("Unknown command".to_string(), cmd_mode);
                }
            }
        } else {
            match command {
                "q" => {
                    tx.send("serverlogs".to_string()).unwrap();
                    cmd_mode = true;
                    winconsole::console::clear().unwrap();
                    for line in &cmd_logs.log {
                        println!("{}", *line);
                    }
                }
                _ => (),
            }
        }
    }
}