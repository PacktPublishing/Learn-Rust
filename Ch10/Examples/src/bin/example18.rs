use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(b"Hello, server!")?;

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Server replied: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    Ok(())
}
