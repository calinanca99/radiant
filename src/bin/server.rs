use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle(mut s: TcpStream) {
    let mut buf = [0; 1024];
    let b = s.read(&mut buf).unwrap();

    let msg = String::from_utf8(buf[..b].to_vec()).unwrap();
    println!("Client says: {msg}");
    s.write_all(b"world").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for conn in listener.incoming() {
        handle(conn.unwrap());
    }
}
