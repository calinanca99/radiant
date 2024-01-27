use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut s: TcpStream) {
    loop {
        let mut buf = [0; 1024];

        match s.read(&mut buf) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(b) => {
                let msg = String::from_utf8(buf[..b].to_vec()).unwrap();
                println!("Client says: {msg}");
                s.write_all(b"world").unwrap();
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
