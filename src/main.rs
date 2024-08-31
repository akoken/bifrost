mod resp;
mod server;

use server::Server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    let server = Server::new(listener);
    server.start().await?;

    Ok(())
}
