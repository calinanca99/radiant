use std::{
    io::{Cursor, Read, Write},
    net::TcpStream,
};

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    frame::{Frame, FrameError},
    RadiantError, HEADER_SIZE, MAX_MESSAGE_LENGTH,
};

pub struct ConnectionHandler {
    stream: TcpStream,
    read_buffer: BytesMut,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            // If the buffered is not zeroed, then the blocking version of `read`
            // will always return 0 even if there's data to be read from the socket.
            read_buffer: BytesMut::zeroed(HEADER_SIZE + MAX_MESSAGE_LENGTH),
        }
    }

    pub fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            // The call to `read` will block until some data is read from
            // the socket.
            if 0 == self.stream.read(&mut self.read_buffer)? {
                // Hack used instead of `self.read_buffer.is_empty`. Needed
                // because the read_buffer is zeroed in the beginning.
                if self.read_buffer.iter().all(|x| *x == 0) {
                    return Ok(None);
                } else {
                    return Err(crate::RadiantError::NetworkError(
                        "connection reset by peer".into(),
                    ));
                }
            }

            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        let mut cursor = Cursor::new(&self.read_buffer[..]);

        match Frame::check(&mut cursor) {
            Ok(_) => {
                // Given that `Frame::check` advanced the cursor, the cursor
                // must be reset to the beginning. Otherwise, it's not possible
                // to use it for parsing.
                let len = cursor.position() as usize;
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;

                // Discard data from the read buffer
                self.read_buffer.advance(len);

                Ok(Some(frame))
            }
            Err(FrameError::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn send_message(&mut self, msg: &str) -> crate::Result<()> {
        let len = msg.len();
        if len > MAX_MESSAGE_LENGTH {
            return Err(RadiantError::MessageLimit);
        }

        let mut write_buf = BytesMut::with_capacity(HEADER_SIZE + len);

        write_buf.put_u32(len as u32);
        write_buf.put_slice(msg.as_bytes());

        match self.stream.write_all(&write_buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
