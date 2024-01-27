use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    stream.write_all(b"hello").unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("Server says: {buf}");
}
