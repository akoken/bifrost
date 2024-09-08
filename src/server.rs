use crate::{frame::RespCodec, resp::RespType};
use futures::{SinkExt, StreamExt};
use std::io;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(listener: TcpListener) -> Server {
        Server { listener }
    }

    pub async fn start(&self) -> io::Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("New connection from {}", addr);

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut framed = Framed::new(stream, RespCodec);

    while let Some(result) = framed.next().await {
        match result {
            Ok(request) => {
                let response = process_request(request);
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

fn process_request(request: RespType) -> RespType {
    // Here you would implement your command processing logic
    // For now, we'll just echo the request back
    match request {
        RespType::Array(array) => {
            if let Some(RespType::BulkString(command)) = array.first() {
                match command.to_uppercase().as_str() {
                    "PING" => RespType::SimpleString("PONG".to_string()),
                    "ECHO" => {
                        if let Some(RespType::BulkString(message)) = array.get(1) {
                            RespType::BulkString(message.clone())
                        } else {
                            RespType::Error(
                                "ERR wrong number of arguments for 'echo' command".to_string(),
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
