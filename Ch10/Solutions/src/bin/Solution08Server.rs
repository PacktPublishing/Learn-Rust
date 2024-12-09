use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::io;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client connected: {}", peer_addr);

    let mut reader = BufReader::new(&stream);
    let mut message = String::new();

    // Read the message from the client
    reader.read_line(&mut message)?;
    let message = message.trim(); // Remove any trailing whitespace or newline

    // Calculate the word count
    let word_count = message.split_whitespace().count();

    // Log the client's message and the word count
    {
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")?;
        writeln!(log_file, "Client: {}, Message: '{}', Word Count: {}", peer_addr, message, word_count)?;
    }

    // Send the word count back to the client
    let response = format!("Word count: {}\n", word_count);
    stream.write_all(response.as_bytes())?;
    println!("Processed message from {}: '{}'", peer_addr, message);

    Ok(())
}

fn main() -> io::Result<()> {
    // Start the server
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is listening on 127.0.0.1:8080");

    // Accept a single connection
    if let Ok((stream, _addr)) = listener.accept() {
        println!("Accepted a connection.");
        handle_client(stream)?;
    }

    println!("Server shutting down after handling one connection.");

    Ok(())
}
