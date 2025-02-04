mod ping;
mod echo;
mod get;
mod set;
mod del;
mod exists;
mod incr;
mod decr;

pub use ping::PingCommand;
pub use echo::EchoCommand;
pub use get::GetCommand;
pub use set::SetCommand;
pub use del::DelCommand;
pub use exists::ExistsCommand;
pub use incr::IncrCommand;
pub use decr::DecrCommand;

use crate::resp::RespType;
use crate::storage::db::Db;

pub trait Command {
    fn execute(&self, db: &Db) -> RespType;
} 