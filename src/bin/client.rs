use std::{io::Read, net::TcpStream};

use radiant::{parse_message, write_message};

fn query(stream: &mut TcpStream, msg: &str) {
    write_message(stream, msg).unwrap();

    let mut data = vec![0; 4 + 4096];
    // TODO: Do something with the read amount (advice from clippy)
    let _ = stream.read(&mut data).unwrap();

    let msg = parse_message(data.as_slice()).unwrap();

    println!("Server says: {msg}");
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    query(&mut stream, "hello1");
    query(&mut stream, "hello2");
    query(&mut stream, "hello3");
}
