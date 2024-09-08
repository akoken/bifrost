use bytes::Buf;
use bytes::BytesMut;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

use crate::resp::Resp;
use crate::resp::RespError;
use crate::resp::RespType;

pub struct RespCodec;

impl Decoder for RespCodec {
    type Item = RespType;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        let mut resp = Resp::new(src.clone());
        match resp.parse() {
            Ok((resp_type, consumed)) => {
                src.advance(consumed);
                Ok(Some(resp_type))
            }
            Err(RespError::Incomplete) => Ok(None),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
        }
    }
}

impl Encoder<RespType> for RespCodec {
    type Error = io::Error;

    fn encode(&mut self, item: RespType, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item.to_bytes());
        Ok(())
    }
}
