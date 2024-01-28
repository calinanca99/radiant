pub mod connection_handler;

pub mod frame;
use frame::FrameError;

pub const HEADER_SIZE: usize = 4;
pub const MAX_MESSAGE_LENGTH: usize = 4096; // 4MB

pub type Result<T> = std::result::Result<T, RadiantError>;

#[derive(thiserror::Error, Debug)]
pub enum RadiantError {
    #[error("IO error: {0}")]
    IOError(String),
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("message length must be lower than {} bytes", MAX_MESSAGE_LENGTH)]
    MessageLimit,
    #[error("cannot parse message")]
    ParseError,
    #[error("other error")]
    Other,
}

impl From<FrameError> for RadiantError {
    fn from(value: FrameError) -> Self {
        match value {
            FrameError::Incomplete => Self::Other,
            FrameError::Parse => Self::ParseError,
            FrameError::TooLarge => Self::MessageLimit,
        }
    }
}

impl From<std::io::Error> for RadiantError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}
