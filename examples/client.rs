use std::{
    io::{Read, Write},
    net::TcpStream,
};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use radiant::{RadiantError, Request, Response, DEFAULT_CAPACITY, HEADER_SIZE};

fn write_message(stream: &mut TcpStream, msg: &str) -> radiant::Result<()> {
    let len = msg.len();

    let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

    write_buf.put_u32(len as u32);
    write_buf.put_slice(msg.as_bytes());

    match stream.write_all(&write_buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn parse_message(data: &[u8]) -> radiant::Result<Response> {
    let mut read_buf = Bytes::copy_from_slice(data);
    let len = read_buf.get_u32() as usize;

    // TODO: Check `read_buf`.remaining() > `length`
    // If that's not the case then issue another `read` from the socket
    // and see if there's enough data to parse a whole message
    //
    // Ideally there'd be a `read_message` function that manages the TcpStream
    // and some data buffer associated with the stream
    let message_bytes = read_buf.copy_to_bytes(len);

    match String::from_utf8(message_bytes.to_vec()) {
        Ok(s) => Ok(Response::from_str(s.as_str())?),
        Err(e) => Err(RadiantError::ParseError(e.to_string())),
    }
}

fn query(stream: &mut TcpStream, msg: &str) -> Response {
    write_message(stream, msg).unwrap();

    let mut data = vec![0; HEADER_SIZE + DEFAULT_CAPACITY];
    // TODO: Do something with the read amount (advice from clippy)
    let _ = stream.read(&mut data).unwrap();

    parse_message(data.as_slice()).unwrap()
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let req = Request::Ping;
    println!("Sending: {:?}", req);
    let res = query(&mut stream, req.to_string().unwrap().as_str());
    println!("Server says: {:?}", res);

    let req = Request::Set("foo".to_string(), "bar".as_bytes().to_vec());
    println!("Sending: {:?}", req);
    let res = query(&mut stream, req.to_string().unwrap().as_str());
    println!("Server says: {:?}", res);

    let req = Request::Get("foo".to_string());
    println!("Sending: {:?}", req);
    let res = query(&mut stream, req.to_string().unwrap().as_str());
    println!("Server says: {:?}", res);
}
