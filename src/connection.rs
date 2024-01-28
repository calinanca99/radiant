use std::io::Cursor;

use bytes::{Buf, BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{Frame, FrameError, RadiantError, HEADER_SIZE, MAX_MESSAGE_LENGTH};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(HEADER_SIZE + MAX_MESSAGE_LENGTH),
        }
    }

    pub async fn process(&mut self) -> crate::Result<()> {
        let peer_addr = match self.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => return Err(RadiantError::NetworkError(e.to_string())),
        };
        println!("Incoming connection: {}", peer_addr);

        loop {
            match self.read_frame().await {
                Ok(Some(frame)) => {
                    println!("Client says: {frame}");
                    if let Err(e) = self.send_message("world").await {
                        eprintln!("{e}");
                        continue;
                    }
                }
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

    async fn send_message(&mut self, msg: &str) -> crate::Result<()> {
        let len = msg.len();
        if len > MAX_MESSAGE_LENGTH {
            return Err(RadiantError::MessageLimit);
        }

        let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

        write_buf.put_u32(len as u32);
        write_buf.put_slice(msg.as_bytes());

        match self.stream.write_all(&write_buf).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
