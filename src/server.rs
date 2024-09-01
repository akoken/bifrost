use crate::resp::{Resp, RespType};
use bytes::BytesMut;
use std::io::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(listener: TcpListener) -> Server {
        Server { listener }
    }

    pub async fn start(&self) -> Result<()> {
        loop {
            let mut stream = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(_) => panic!("Connection Error"),
            };

            println!("New connection from {}", stream.peer_addr()?);

            tokio::spawn(async move {
                let mut buffer = BytesMut::with_capacity(1024);
                let _ = stream.read_buf(&mut buffer).await;

                let mut parser = Resp::new(buffer);
                let resp_data = match parser.parse() {
                    Ok((data, _)) => data,
                    Err(e) => RespType::Error(format!("{}", e)),
                };

                if let Err(e) = &mut stream.write_all(&resp_data.to_bytes()[..]).await {
                    panic!("Error writing response: {}", e);
                }
            });
        }
    }

    pub async fn accept_conn(&self) -> Result<TcpStream> {
        match self.listener.accept().await {
            Ok((stream, _)) => Ok(stream),
            Err(e) => Err(e),
        }
    }
}
