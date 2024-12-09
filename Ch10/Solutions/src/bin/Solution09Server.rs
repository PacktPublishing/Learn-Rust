use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Bind the server to a specific address and port
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("UDP server listening on 127.0.0.1:8080");

    let mut buffer = [0; 1024];
    loop {
        // Receive a message from the client
        let (bytes_received, src) = socket.recv_from(&mut buffer)?;
        let received_msg = String::from_utf8_lossy(&buffer[..bytes_received]);

        println!("Received '{}' from {}", received_msg, src);

        // Compute the length of the message
        let response = format!("Message length: {}", bytes_received);

        // Send the response back to the client
        socket.send_to(response.as_bytes(), src)?;
    }
}
