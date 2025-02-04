use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct GetCommand(pub String);

impl Command for GetCommand {
    fn execute(&self, db: &Db) -> RespType {
        db.get(&self.0).unwrap_or(RespType::Null)
    }
} 