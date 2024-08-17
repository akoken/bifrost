use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000".to_string();
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_connection(stream);
            }
            Err(e) => {
                println!("Connection failed:{}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut received: Vec<u8> = vec![];
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        received.extend_from_slice(&buffer[..bytes_read]);

        if bytes_read < 1024 {
            break;
        }
    }

    println!("Response:{}", String::from_utf8_lossy(&received));
    stream.write_all(&received)?;

    Ok(())
}
