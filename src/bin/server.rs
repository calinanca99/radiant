use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use radiant::{parse_message, write_message};

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut data = vec![0; 4 + 4096];

        match stream.read(&mut data) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(_) => {
                let msg = parse_message(data.as_slice()).unwrap();
                println!("Client says: {msg}");
                write_message(&mut stream, "world").unwrap();
            }
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("{e}"),
        };
    }
}
