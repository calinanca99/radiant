use std::{fmt::Display, io::Cursor};

use bytes::Buf;

use crate::MAX_MESSAGE_LENGTH;

#[derive(Debug)]
pub struct Frame(String);

impl Frame {
    pub fn check(buf: &mut Cursor<&[u8]>) -> Result<(), FrameError> {
        let length = get_u32(buf)? as usize;

        if length > MAX_MESSAGE_LENGTH {
            return Err(FrameError::TooLarge);
        }

        if buf.remaining() < length {
            Err(FrameError::Incomplete)
        } else {
            buf.advance(length);
            Ok(())
        }
    }

    /// Must be used only after [`Frame::check`] returned `Ok()`. Before
    /// using, the caller must reset `buf` to the original cursor position
    /// before [`Frame::check`] was called.
    pub fn parse(buf: &mut Cursor<&[u8]>) -> Result<Self, FrameError> {
        let length = buf.get_u32() as usize;

        let mut dst = vec![0; length];
        buf.copy_to_slice(&mut dst);

        match String::from_utf8(dst) {
            Ok(s) => Ok(Frame(s)),
            Err(_) => Err(FrameError::Parse),
        }
    }

    pub fn into(self) -> String {
        self.0
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn get_u32(src: &mut Cursor<&[u8]>) -> Result<u32, FrameError> {
    if src.remaining() < 4 {
        return Err(FrameError::Incomplete);
    }

    Ok(src.get_u32())
}

#[derive(thiserror::Error, Debug)]
pub enum FrameError {
    #[error("insufficient data")]
    Incomplete,
    #[error("cannot parse frame")]
    Parse,
    #[error("message is too large")]
    TooLarge,
}
