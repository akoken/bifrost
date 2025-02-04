use crate::resp::RespType;
use crate::storage::db::Db;
use super::Command;

pub struct SetCommand {
    pub key: String,
    pub value: RespType,
}

impl Command for SetCommand {
    fn execute(&self, db: &Db) -> RespType {
        db.set(self.key.clone(), self.value.clone())
    }
} 