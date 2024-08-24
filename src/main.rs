use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("New connection from {}", stream.peer_addr()?);

        // Handle the connection here
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut stream).await {
                println!("An error occurred: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = vec![0; 1024];

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

    Ok(())
}
