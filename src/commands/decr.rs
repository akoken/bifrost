use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct DecrCommand(pub String);

impl Command for DecrCommand {
    fn execute(&self, db: &Db) -> RespType {
        db.decr(&self.0)
    }
} 