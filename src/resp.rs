use bytes::BytesMut;

const STRING: char = '+';
const ERROR: char = '-';
const INTEGER: char = ':';
const BULK: char = '$';
const ARRAY: char = '*';

#[derive(Clone, Debug)]
pub enum RespType {
    BulkString(String),
    SimpleString(String),
    SimpleError(String),
}

#[derive(Debug)]
pub enum RespError {
    InvalidBulkString(String),
    InvalidSimpleString(String),
    Other(String),
}

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespError::Other(msg) => msg.as_str().fmt(f),
            RespError::InvalidBulkString(msg) => msg.as_str().fmt(f),
            RespError::InvalidSimpleString(msg) => msg.as_str().fmt(f),
        }
    }
}

pub struct Resp {
    buffer: BytesMut,
}

impl Resp {
    pub fn new(buffer: BytesMut) -> Self {
        Resp { buffer }
    }
}
