use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct DelCommand(pub String);

impl Command for DelCommand {
    fn execute(&self, db: &Db) -> RespType {
        db.del(&self.0)
    }
} 