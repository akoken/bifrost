use crate::resp::RespType;
use crate::storage::db::Db;
use crate::error::BifrostError;
use super::Command;

pub struct DecrCommand(pub String);

impl Command for DecrCommand {
    fn execute(&self, db: &Db) -> RespType {
        match db.decr(&self.0) {
            Ok(resp) => resp,
            Err(e) => match e {
                BifrostError::StorageError(msg) => RespType::Error(msg),
                _ => RespType::Error("ERR internal error".to_string()),
            }
        }
    }
} 