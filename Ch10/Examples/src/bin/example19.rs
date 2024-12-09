use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut stream = stream;

    loop {
        let mut message = String::new();
        let bytes_read = reader.read_line(&mut message)?;
        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }
        println!("Received: {}", message.trim());

        let response = format!("Server received -> {}\n", message.trim());
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server is listening on port 7878");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream).ok();
        }
        break;
    }

    Ok(())
}
