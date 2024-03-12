use std::fmt::Display;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use server::{Request, Response, DEFAULT_CAPACITY, HEADER_SIZE};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("cannot establish connection")]
    ConnectionError,
    #[error("cannot serialize data")]
    SerializationError,
    #[error("cannot deserialize data")]
    DeserializationError,
    #[error("cannot read or write to stream")]
    IOError,
    #[error("server error")]
    Server,
}

pub type Result<T> = std::result::Result<T, ClientError>;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let conn = match TcpStream::connect(addr).await {
            Ok(conn) => conn,
            Err(_) => return Err(ClientError::ConnectionError),
        };
        Ok(Self { stream: conn })
    }

    pub async fn set(&mut self, key: impl Display, value: impl Serialize) -> Result<()> {
        let data = match serde_json::to_string(&value) {
            Ok(v) => v,
            Err(_) => return Err(ClientError::SerializationError),
        };
        let request = Request::Set(key.to_string(), data.as_bytes().to_vec());
        let msg = match serde_json::to_string(&request) {
            Ok(v) => v,
            Err(_) => return Err(ClientError::SerializationError),
        };

        let len = msg.len();
        let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

        write_buf.put_u32(len as u32);
        write_buf.put_slice(msg.as_bytes());

        match self.stream.write_all(&write_buf).await {
            Ok(_) => (),
            Err(_) => return Err(ClientError::IOError),
        };

        let mut data = vec![0; HEADER_SIZE + DEFAULT_CAPACITY];
        let _ = self.stream.read(&mut data).await.unwrap();

        let mut read_buf = Bytes::copy_from_slice(&data);
        let len = read_buf.get_u32() as usize;
        let message_bytes = read_buf.copy_to_bytes(len);

        let s = match String::from_utf8(message_bytes.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ClientError::DeserializationError),
        };

        let res: Response = match serde_json::from_str(&s) {
            Ok(v) => v,
            Err(_) => return Err(ClientError::DeserializationError),
        };

        match res {
            Response::Ok => Ok(()),
            Response::Error(_) => Err(ClientError::Server),
            _ => unreachable!(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&mut self, key: impl Display) -> Result<T> {
        let request = Request::Get(key.to_string());
        let msg = match serde_json::to_string(&request) {
            Ok(v) => v,
            Err(_) => return Err(ClientError::SerializationError),
        };

        let len = msg.len();
        let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

        write_buf.put_u32(len as u32);
        write_buf.put_slice(msg.as_bytes());

        match self.stream.write_all(&write_buf).await {
            Ok(_) => (),
            Err(_) => return Err(ClientError::IOError),
        };

        let mut data = vec![0; HEADER_SIZE + DEFAULT_CAPACITY];
        let _ = self.stream.read(&mut data).await.unwrap();

        let mut read_buf = Bytes::copy_from_slice(&data);
        let len = read_buf.get_u32() as usize;
        let message_bytes = read_buf.copy_to_bytes(len);

        let s = match String::from_utf8(message_bytes.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ClientError::DeserializationError),
        };
        let res: Response = match serde_json::from_str(&s) {
            Ok(v) => v,
            Err(_) => return Err(ClientError::DeserializationError),
        };
        let data = match res {
            Response::Error(_) => return Err(ClientError::Server),
            Response::Get(_, data) => data,
            _ => unreachable!(),
        };

        let s = match String::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(ClientError::DeserializationError),
        };
        match serde_json::from_str(&s) {
            Ok(v) => Ok(v),
            Err(_) => Err(ClientError::DeserializationError),
        }
    }
}
