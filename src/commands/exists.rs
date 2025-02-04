use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct ExistsCommand(pub String);

impl Command for ExistsCommand {
    fn execute(&self, db: &Db) -> RespType {
        db.exists(&self.0)
    }
} 