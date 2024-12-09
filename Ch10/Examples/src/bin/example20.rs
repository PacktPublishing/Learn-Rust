use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    let requests = ["Hello, server!", "Another request", "Final message"];
    for request in &requests {
        stream.write_all(request.as_bytes())?;
        stream.write_all(b"\n")?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("Server replied: {}", response.trim());
    }

    Ok(())
}
