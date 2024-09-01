use bytes::{Bytes, BytesMut};

const STRING: u8 = b'+';
const ERROR: u8 = b'-';
const INTEGER: u8 = b':';
const BULK: u8 = b'$';
const ARRAY: u8 = b'*';

#[derive(Clone, Debug)]
pub enum RespType {
    BulkString(String),
    SimpleString(String),
    Error(String),
}

impl RespType {
    pub fn to_bytes(&self) -> Bytes {
        match self {
            RespType::SimpleString(ss) => Bytes::from_iter(format!("+{}\r\n", ss).into_bytes()),
            RespType::BulkString(bs) => {
                let bulkstr_bytes = format!("${}\r\n{}\r\n", bs.chars().count(), bs).into_bytes();
                Bytes::from_iter(bulkstr_bytes)
            }
            RespType::Error(es) => Bytes::from_iter(format!("-{}\r\n", es).into_bytes()),
        }
    }
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

    pub fn parse(&mut self) -> Result<(RespType, usize), RespError> {
        let typ = self.buffer[0];

        println!("Type is:{}", &typ);
        println!(
            "Incoming data:{:?}",
            String::from_utf8(self.buffer.to_vec())
        );

        match typ {
            BULK => Self::parse_bulk_string(self),
            _ => Err(RespError::Other(String::from("Invalid RESP data type!"))),
        }
    }

    fn parse_bulk_string(&mut self) -> Result<(RespType, usize), RespError> {
        let (bulkstr_len, bytes_consumed) =
            if let Some((data, len)) = Self::read_line(&self.buffer[1..]) {
                let bulkstr_len = Self::parse_length(data)?;
                (bulkstr_len, len + 1)
            } else {
                return Err(RespError::InvalidBulkString(String::from(
                    "Invalid value for bulk string",
                )));
            };

        let bulkstr_end_idx = bytes_consumed + bulkstr_len;
        if bulkstr_end_idx >= self.buffer.len() {
            return Err(RespError::InvalidBulkString(String::from(
                "Invalid value for bulk string length",
            )));
        }

        let bulkstr = String::from_utf8(self.buffer[bytes_consumed..bulkstr_end_idx].to_vec());
        println!("Bulkstring:{:?}", &bulkstr);

        match bulkstr {
            Ok(bs) => Ok((RespType::BulkString(bs), bulkstr_end_idx + 2)),
            Err(_) => Err(RespError::InvalidBulkString(String::from(
                "Bulk string value is not a valid UTF-8 string",
            ))),
        }
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
