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

    fn read_line(buf: &[u8]) -> Option<(&[u8], usize)> {
        for i in 1..buf.len() {
            if buf[i - 1] == b'\r' && buf[i] == b'\n' {
                return Some((&buf[0..(i - 1)], i + 1));
            }
        }

        None
    }

    fn parse_length(buf: &[u8]) -> Result<usize, RespError> {
        let utf8_str = String::from_utf8(buf.to_vec());
        match utf8_str {
            Ok(s) => {
                let int = s.parse::<usize>();
                match int {
                    Ok(n) => Ok(n),
                    Err(_) => Err(RespError::Other(String::from(
                        "Invalid value for an integer",
                    ))),
                }
            }
            Err(_) => Err(RespError::Other(String::from("Invalid UTF-8 string"))),
        }
    }
}
