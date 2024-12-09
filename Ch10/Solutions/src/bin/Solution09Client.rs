use std::net::UdpSocket;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Bind the client to any available port
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Client bound to {}", socket.local_addr()?);

    // Connect to the server
    socket.connect("127.0.0.1:8080")?;

    // Prompt the user for a message
    print!("Enter a message to send: ");
    io::stdout().flush()?;
    let mut message = String::new();
    io::stdin().read_line(&mut message)?;

    // Send the message to the server
    socket.send(message.as_bytes())?;
    println!("Sent: {}", message.trim());

    // Receive the server's response
    let mut buffer = [0; 1024];
    let bytes_received = socket.recv(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_received]);

    println!("Server replied: {}", response);

    Ok(())
}
