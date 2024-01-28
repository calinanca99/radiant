use std::{
    io::{Read, Write},
    net::TcpStream,
};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use radiant::{RadiantError, HEADER_SIZE, MAX_MESSAGE_LENGTH};

fn write_message(stream: &mut TcpStream, msg: &str) -> radiant::Result<()> {
    let len = msg.len();
    if len > MAX_MESSAGE_LENGTH {
        return Err(RadiantError::MessageLimit);
    }

    let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

    write_buf.put_u32(len as u32);
    write_buf.put_slice(msg.as_bytes());

    match stream.write_all(&write_buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn parse_message(data: &[u8]) -> radiant::Result<String> {
    let mut read_buf = Bytes::copy_from_slice(data);
    let len = read_buf.get_u32() as usize;
    if len > MAX_MESSAGE_LENGTH {
        return Err(RadiantError::MessageLimit);
    }

    // TODO: Check `read_buf`.remaining() > `length`
    // If that's not the case then issue another `read` from the socket
    // and see if there's enough data to parse a whole message
    //
    // Ideally there'd be a `read_message` function that manages the TcpStream
    // and some data buffer associated with the stream
    let message_bytes = read_buf.copy_to_bytes(len);

    match String::from_utf8(message_bytes.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => Err(RadiantError::ParseError),
    }
}

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
