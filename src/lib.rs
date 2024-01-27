use std::{io::Write, net::TcpStream};

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub const HEADER_SIZE: usize = 4;
pub const MAX_MESSAGE_LENGTH: usize = 4096; // 4MB

pub type Result<T> = std::result::Result<T, RadiantError>;

#[derive(thiserror::Error, Debug)]
pub enum RadiantError {
    #[error("message length must be lower than {} bytes", MAX_MESSAGE_LENGTH)]
    MessageLimit,
    #[error("cannot write message: {0}")]
    WriteError(String),
    #[error("cannot parse message: {0}")]
    ParseError(String),
}

pub fn write_message(stream: &mut TcpStream, msg: &str) -> Result<()> {
    let len = msg.len();
    if len > MAX_MESSAGE_LENGTH {
        return Err(RadiantError::MessageLimit);
    }

    let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

    write_buf.put_u32(len as u32);
    write_buf.put_slice(msg.as_bytes());

    match stream.write_all(&write_buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(RadiantError::WriteError(e.to_string())),
    }
}

pub fn parse_message(data: &[u8]) -> Result<String> {
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
        Err(e) => Err(RadiantError::ParseError(e.to_string())),
    }
}
