use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Connect to the server
    let server_address = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(server_address)?;
    println!("Connected to server at {}", server_address);

    // Get a message from the user
    print!("Enter a message: ");
    io::stdout().flush()?; // Ensure the prompt is displayed immediately

    let mut message = String::new();
    io::stdin().read_line(&mut message)?;
    let message = message.trim(); // Remove any trailing newline

    // Send the message to the server
    stream.write_all(message.as_bytes())?;
    stream.write_all(b"\n")?; // Add a newline to mark the end of the message
    println!("Message sent: {}", message);

    // Read the server's response
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Server replied: {}", response.trim());

    Ok(())
}
