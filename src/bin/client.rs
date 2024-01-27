use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn query(stream: &mut TcpStream, msg: &str) {
    stream.write_all(msg.as_bytes()).unwrap();

    let mut buf = [0; 1024];
    let b = stream.read(&mut buf).unwrap();
    let msg = String::from_utf8(buf[..b].to_vec()).unwrap();

    println!("Server says: {msg}");
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    query(&mut stream, "hello1");
    query(&mut stream, "hello2");
    query(&mut stream, "hello3");
}
