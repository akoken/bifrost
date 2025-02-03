use crate::storage::db::Db;
use crate::{frame::RespCodec, resp::RespType};

use futures::{SinkExt, StreamExt};
use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

pub struct Server {
    listener: TcpListener,
    db: Arc<Db>,
}

impl Server {
    pub fn new(listener: TcpListener) -> Server {
        Server {
            listener,
            db: Arc::new(Db::new()),
        }
    }

    pub async fn start(self) -> io::Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("New connection from {}", addr);

            let db_clone = Arc::clone(&self.db);

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, &db_clone).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(stream: TcpStream, db: &Arc<Db>) -> io::Result<()> {
    let mut framed = Framed::new(stream, RespCodec);

    while let Some(result) = framed.next().await {
        match result {
            Ok(request) => {
                let response = process_request(request, db);
                framed.send(response).await?;
            }
            Err(e) => {
                eprintln!("Error decoding frame: {}", e);
                let error_response = RespType::Error(format!("Error: {}", e));
                framed.send(error_response).await?;
            }
        }
    }

    Ok(())
}

fn process_request(request: RespType, db: &Db) -> RespType {
    match request {
        RespType::Array(array) => {
            if let Some(RespType::BulkString(command)) = array.first() {
                match command.to_uppercase().as_str() {
                    "PING" => RespType::SimpleString("PONG".to_string()),
                    "ECHO" => {
                        if let Some(message) = array.get(1) {
                            message.clone()
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'echo' command".to_string(),
                            )
                        }
                    }
                    "GET" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            db.get(key).unwrap_or(RespType::Null)
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'get' command".to_string(),
                            )
                        }
                    }
                    "SET" => {
                        if let (Some(RespType::BulkString(key)), Some(value)) =
                            (array.get(1), array.get(2))
                        {
                            db.set(key.clone(), value.clone())
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'set' command".to_string(),
                            )
                        }
                    }
                    "DEL" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            db.del(key)
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'del' command".to_string(),
                            )
                        }
                    }
                    "EXISTS" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            db.exists(key)
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'exists' command".to_string(),
                            )
                        }
                    }
                    "INCR" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            db.incr(key)
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'incr' command".to_string(),
                            )
                        }
                    }
                    "DECR" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            db.decr(key)
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'decr' command".to_string(),
                            )
                        }
                    }
                    _ => RespType::Error("ERR unknown command".to_string()),
                }
            } else {
                RespType::Error("ERR invalid request".to_string())
            }
        }
        _ => RespType::Error("ERR invalid request".to_string()),
    }
}
