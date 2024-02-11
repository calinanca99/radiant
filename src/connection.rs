use std::io::Cursor;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    Db, Frame, FrameError, RadiantError, Request, Response, DEFAULT_CAPACITY, HEADER_SIZE,
};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    db: Db,
}

impl Connection {
    pub fn new(stream: TcpStream, db: Db) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(HEADER_SIZE + DEFAULT_CAPACITY),
            db,
        }
    }

    pub async fn process(&mut self) -> crate::Result<()> {
        let peer_addr = match self.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => return Err(RadiantError::NetworkError(e.to_string())),
        };
        println!("Incoming connection: {}", peer_addr);

        loop {
            let frame = match self.read_frame().await {
                Ok(Some(frame)) => frame,
                Ok(None) => {
                    println!("Client {} disconnected", peer_addr);
                    break;
                }
                Err(crate::RadiantError::NetworkError(_)) => {
                    eprintln!("Connection reset by peer");
                    break;
                }
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            // TODO: `match` and send `Response::Error`
            let command = Request::from_frame(&frame)?;
            if let Err(e) = self.process_command(command).await {
                eprintln!("{e}");
                continue;
            }
        }

        Ok(())
    }

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(crate::RadiantError::NetworkError(
                        "connection reset by peer".into(),
                    ));
                }
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        let mut cursor = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut cursor) {
            Ok(_) => {
                // Given that `Frame::check` advanced the cursor, the cursor
                // must be reset to the beginning. Otherwise, it's not possible
                // to use it for parsing.
                let len = cursor.position() as usize;
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;

                // Discard data from the read buffer.
                self.buffer.advance(len);

                Ok(Some(frame))
            }
            Err(FrameError::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn process_command(&mut self, command: Request) -> crate::Result<()> {
        match command {
            Request::Ping => {
                let msg = Response::Pong.to_string()?;
                self.send_message(&msg).await?
            }
            Request::Get(key) => {
                let value = match self.db.get(&key) {
                    Some(v) => v.to_vec(),
                    None => vec![],
                };

                let msg = Response::Get(key, value).to_string()?;
                self.send_message(&msg).await?
            }
            Request::Set(key, value) => {
                self.db.set(key, Bytes::from(value));

                let msg = Response::Ok.to_string()?;
                self.send_message(&msg).await?
            }
        }

        Ok(())
    }

    async fn send_message(&mut self, msg: &str) -> crate::Result<()> {
        let len = msg.len();

        let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

        write_buf.put_u32(len as u32);
        write_buf.put_slice(msg.as_bytes());

        match self.stream.write_all(&write_buf).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
