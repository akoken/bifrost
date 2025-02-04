use bytes::{Bytes, BytesMut};

const STRING: u8 = b'+';
const ERROR: u8 = b'-';
const INTEGER: u8 = b':';
const BULK: u8 = b'$';
const ARRAY: u8 = b'*';

#[derive(Debug, Clone, PartialEq)]
pub enum RespType {
    BulkString(String),
    SimpleString(String),
    Integer(i64),
    Array(Vec<RespType>),
    Error(String),
    Null,
}

impl RespType {
    pub fn to_bytes(&self) -> Bytes {
        match self {
            RespType::BulkString(bs) => {
                let bulkstr_bytes = format!("${}\r\n{}\r\n", bs.chars().count(), bs).into_bytes();
                Bytes::from_iter(bulkstr_bytes)
            }
            RespType::SimpleString(ss) => Bytes::from_iter(format!("+{}\r\n", ss).into_bytes()),
            RespType::Integer(i) => Bytes::from(format!(":{}\r\n", i)),
            RespType::Array(arr) => {
                let mut result = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    result.extend_from_slice(&item.to_bytes());
                }
                Bytes::from(result)
            }
            RespType::Error(es) => Bytes::from_iter(format!("-{}\r\n", es).into_bytes()),
            RespType::Null => Bytes::from("$-1\r\n"),
        }
    }
}

impl PartialEq for RespType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RespType::BulkString(a), RespType::BulkString(b)) => a == b,
            (RespType::SimpleString(a), RespType::SimpleString(b)) => a == b,
            (RespType::Integer(a), RespType::Integer(b)) => a == b,
            (RespType::Array(a), RespType::Array(b)) => a == b,
            (RespType::Error(a), RespType::Error(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum RespError {
    InvalidBulkString(String),
    InvalidSimpleString(String),
    InvalidInteger(String),
    Incomplete,
    Other(String),
}

impl std::error::Error for RespError {}

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespError::InvalidBulkString(msg) => write!(f, "Invalid bulk string: {}", msg),
            RespError::InvalidSimpleString(msg) => write!(f, "Invalid simple string: {}", msg),
            RespError::InvalidInteger(msg) => write!(f, "Invalid integer: {}", msg),
            RespError::Incomplete => write!(f, "Incomplete RESP data"),
            RespError::Other(msg) => write!(f, "Other error: {}", msg),
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
        match self.buffer[0] {
            BULK => Self::parse_bulk_string(self),
            STRING => Self::parse_simple_string(self),
            INTEGER => Self::parse_integer(self),
            ARRAY => Self::parse_array(self),
            ERROR => Self::parse_error(self),
            _ => Err(RespError::Other(String::from("Invalid RESP data type!"))),
        }
    }

    // $5\r\nhello\r\n
    fn parse_bulk_string(&self) -> Result<(RespType, usize), RespError> {
        let (str_length, mut len) = self.parse_integer_value(1)?;
        if str_length == -1 {
            return Ok((RespType::Null, len));
        }

        let str_length = str_length as usize;

        if self.buffer.len() < len + str_length + 2 {
            return Err(RespError::Incomplete);
        }

        let string = String::from_utf8(self.buffer[len..len + str_length].to_vec())
            .map_err(|_| RespError::InvalidBulkString("Invalid UTF-8".to_string()))?;

        len += str_length + 2; // +2 for \r\n
        Ok((RespType::BulkString(string), len))
    }

    // +OK\r\n
    fn parse_simple_string(&self) -> Result<(RespType, usize), RespError> {
        let (line, len) = self.read_line(1)?;
        Ok((RespType::SimpleString(line), len))
    }

    // :23\r\n
    fn parse_integer(&self) -> Result<(RespType, usize), RespError> {
        let (value, len) = self.parse_integer_value(1)?;
        Ok((RespType::Integer(value), len))
    }

    // *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
    fn parse_array(&self) -> Result<(RespType, usize), RespError> {
        let (arr_size, mut len) = self.parse_integer_value(1)?;
        if arr_size == -1 {
            return Ok((RespType::Null, len));
        }

        let arr_size = arr_size as usize;

        let mut array = Vec::with_capacity(arr_size);
        for _ in 0..arr_size {
            let mut resp = Resp::new(BytesMut::from(&self.buffer[len..]));
            let (item, item_len) = resp.parse()?;
            array.push(item);
            len += item_len;
        }

        Ok((RespType::Array(array), len))
    }

    // -Error message\r\n
    fn parse_error(&self) -> Result<(RespType, usize), RespError> {
        let (line, len) = self.read_line(1)?;
        Ok((RespType::Error(line), len))
    }

    fn read_line(&self, start: usize) -> Result<(String, usize), RespError> {
        for i in start + 1..self.buffer.len() {
            if self.buffer[i - 1] == b'\r' && self.buffer[i] == b'\n' {
                return Ok((
                    String::from_utf8(self.buffer[start..i - 1].to_vec())
                        .map_err(|_| RespError::InvalidSimpleString("Invalid UTF-8".to_string()))?,
                    i + 1,
                ));
            }
        }
        Err(RespError::Incomplete)
    }

    fn parse_integer_value(&self, start: usize) -> Result<(i64, usize), RespError> {
        let (line, pos) = self.read_line(start)?;
        line.parse::<i64>()
            .map_err(|_| RespError::InvalidInteger("Invalid integer format".to_string()))
            .map(|value| (value, pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_resp_eq(
        result: Result<(RespType, usize), RespError>,
        expected: RespType,
        expected_pos: usize,
    ) {
        match result {
            Ok((resp_type, pos)) => {
                assert!(
                    resp_type == expected,
                    "RespType mismatch. Expected {:?}, got {:?}",
                    expected,
                    resp_type
                );
                assert_eq!(
                    pos, expected_pos,
                    "Position mismatch. Expected {}, got {}",
                    expected_pos, pos
                );
            }
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }

    #[test]
    fn test_parse_bulk_string() {
        let mut resp = Resp::new(BytesMut::from("$5\r\nhello\r\n"));
        assert_resp_eq(resp.parse(), RespType::BulkString("hello".to_string()), 11);
    }

    #[test]
    fn test_parse_simple_string() {
        let mut resp = Resp::new(BytesMut::from("+OK\r\n"));
        assert_resp_eq(resp.parse(), RespType::SimpleString("OK".to_string()), 5);
    }

    #[test]
    fn test_parse_integer() {
        let mut resp = Resp::new(BytesMut::from(":23\r\n"));
        assert_resp_eq(resp.parse(), RespType::Integer(23), 5);
    }

    #[test]
    fn test_parse_array() {
        let mut resp = Resp::new(BytesMut::from("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n"));
        assert_resp_eq(
            resp.parse(),
            RespType::Array(vec![
                RespType::BulkString("hello".to_string()),
                RespType::BulkString("world".to_string()),
            ]),
            26,
        );
    }

    #[test]
    fn test_parse_error() {
        let mut resp = Resp::new(BytesMut::from("-Error message\r\n"));
        assert_resp_eq(
            resp.parse(),
            RespType::Error("Error message".to_string()),
            16,
        );
    }

    #[test]
    fn test_parse_null() {
        let input = "$-1\r\n";
        let result = parse_resp(input).unwrap();
        assert_eq!(result, RespType::Null);
    }

    #[test]
    fn test_parse_invalid() {
        let mut resp = Resp::new(BytesMut::from("x5\r\nhello\r\n"));
        assert!(resp.parse().is_err());
    }
}
