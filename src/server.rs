use crate::storage::db::Db;
use crate::{frame::RespCodec, resp::RespType};
use crate::parser::parse_command;
use crate::error::BifrostError;

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
    match parse_command(request) {
        Ok(command) => command.execute(db),
        Err(err) => match err {
            BifrostError::CommandError(msg) | 
            BifrostError::StorageError(msg) |
            BifrostError::ProtocolError(msg) => RespType::Error(msg),
            BifrostError::IoError(e) => RespType::Error(format!("ERR {}", e)),
        }
    }
}
