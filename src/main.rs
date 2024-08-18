use std::{
    io::{BufRead, Write},
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
    let mut reader = std::io::BufReader::new(&mut stream);
    let request = reader.fill_buf()?.to_vec();
    reader.consume(request.len());

    let req = String::from_utf8_lossy(&request);
    println!("Request:{}", &req);

    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    Ok(())
}
