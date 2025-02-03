use crate::resp::RespType;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Db {
    data: Arc<RwLock<HashMap<String, RespType>>>,
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

impl Db {
    pub fn new() -> Self {
        Db {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<RespType> {
        self.data.read().get(key).cloned()
    }

    pub fn set(&self, key: String, value: RespType) -> RespType {
        self.data.write().insert(key, value);
        RespType::SimpleString("OK".to_string())
    }

    pub fn del(&self, key: &str) -> RespType {
        let mut data = self.data.write();
        match data.remove(key) {
            Some(_) => RespType::Integer(1),
            None => RespType::Integer(0),
        }
    }

    pub fn exists(&self, key: &str) -> RespType {
        let exists = self.data.read().contains_key(key);
        RespType::Integer(if exists { 1 } else { 0 })
    }

    pub fn incr(&self, key: &str) -> RespType {
        let mut data = self.data.write();

        match data.get(key) {
            Some(RespType::Integer(value)) => {
                let new_value = value + 1;
                data.insert(key.to_string(), RespType::Integer(new_value));
                RespType::Integer(new_value)
            }
            Some(_) => RespType::Error("ERR value is not an integer".to_string()),
            None => {
                data.insert(key.to_string(), RespType::Integer(1));
                RespType::Integer(1)
            }
        }
    }

    pub fn decr(&self, key: &str) -> RespType {
        let mut data = self.data.write();

        match data.get(key) {
            Some(RespType::Integer(value)) => {
                let new_value = value - 1;
                data.insert(key.to_string(), RespType::Integer(new_value));
                RespType::Integer(new_value)
            }
            Some(_) => RespType::Error("ERR value is not an integer".to_string()),
            None => {
                data.insert(key.to_string(), RespType::Integer(-1));
                RespType::Integer(-1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let db = Db::new();

        // Test SET and GET
        assert_eq!(
            db.set(
                "key1".to_string(),
                RespType::BulkString("value1".to_string())
            ),
            RespType::SimpleString("OK".to_string())
        );

        assert_eq!(
            db.get("key1"),
            Some(RespType::BulkString("value1".to_string()))
        );

        // Test DEL
        assert_eq!(db.del("key1"), RespType::Integer(1));
        assert_eq!(db.get("key1"), None);

        // Test EXISTS
        assert_eq!(db.exists("key1"), RespType::Integer(0));
        db.set(
            "key1".to_string(),
            RespType::BulkString("value1".to_string()),
        );
        assert_eq!(db.exists("key1"), RespType::Integer(1));
    }

    #[test]
    fn test_integer_operations() {
        let db = Db::new();

        // Test INCR
        assert_eq!(db.incr("counter"), RespType::Integer(1));
        assert_eq!(db.incr("counter"), RespType::Integer(2));

        // Test DECR
        assert_eq!(db.decr("counter"), RespType::Integer(1));
        assert_eq!(db.decr("counter"), RespType::Integer(0));
    }
}
