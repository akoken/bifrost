use crate::resp::RespType;
use crate::storage::db::Db;
use crate::error::BifrostError;
use super::Command;

pub struct IncrCommand(pub String);

impl Command for IncrCommand {
    fn execute(&self, db: &Db) -> RespType {
        match db.incr(&self.0) {
            Ok(resp) => resp,
            Err(e) => match e {
                BifrostError::StorageError(msg) => RespType::Error(msg),
                _ => RespType::Error("ERR internal error".to_string()),
            }
        }
    }
} 