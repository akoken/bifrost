use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(&self, _db: &Db) -> RespType {
        RespType::SimpleString("PONG".to_string())
    }
} 