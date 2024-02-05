pub mod connection;
pub use connection::Connection;

mod frame;
use frame::{Frame, FrameError};

pub mod protocol;
pub use protocol::{Request, Response};

pub const HEADER_SIZE: usize = 4;
pub const DEFAULT_CAPACITY: usize = 1024;

pub type Result<T> = std::result::Result<T, RadiantError>;

#[derive(thiserror::Error, Debug)]
pub enum RadiantError {
    #[error("IO error: {0}")]
    IOError(String),
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("parsing error: {0}")]
    ParseError(String),
    #[error("other error")]
    Other,
}

impl From<FrameError> for RadiantError {
    fn from(value: FrameError) -> Self {
        match value {
            FrameError::Incomplete => Self::Other,
            FrameError::Parse(e) => Self::ParseError(e),
        }
    }
}

impl From<std::io::Error> for RadiantError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl From<serde_json::Error> for RadiantError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(value.to_string())
    }
}
