use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct EchoCommand(pub RespType);

impl Command for EchoCommand {
    fn execute(&self, _db: &Db) -> RespType {
        self.0.clone()
    }
} 