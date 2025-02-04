use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BifrostError {
    CommandError(String),
    StorageError(String),
    ProtocolError(String),
    IoError(std::io::Error),
}

impl fmt::Display for BifrostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BifrostError::CommandError(msg) => write!(f, "Command error: {}", msg),
            BifrostError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            BifrostError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            BifrostError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for BifrostError {}

impl From<std::io::Error> for BifrostError {
    fn from(err: std::io::Error) -> Self {
        BifrostError::IoError(err)
    }
} 