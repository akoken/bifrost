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
                let mut buffer = [0; 1024];
                loop {
                    match stream.read(&mut buffer).await {
                        // Return value of `Ok(0)` signifies that the remote has
                        // closed
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => {
                            println!("Received:{:?}", String::from_utf8_lossy(&buffer[..n]));
                            if stream.write_all(&buffer[..n]).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => {
                            break;
                        }
                    }
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
