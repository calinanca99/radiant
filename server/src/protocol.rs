use serde::{Deserialize, Serialize};

use crate::frame::Frame;

#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    Ping,
    Get(String),
    Set(String, Vec<u8>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Pong,
    Ok,
    Error(String),
    Get(String, Vec<u8>),
}

impl Request {
    pub fn from_frame(frame: Frame) -> crate::Result<Self> {
        Ok(serde_json::from_str::<Request>(frame.inner())?)
    }

    pub fn to_string(&self) -> crate::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl Response {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> crate::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn to_string(&self) -> crate::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}
