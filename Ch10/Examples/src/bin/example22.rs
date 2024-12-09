use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Client bound to {}", socket.local_addr()?);

    let server_address = "127.0.0.1:7878";
    let message = "Hello, server!";
    socket.send_to(message.as_bytes(), server_address)?;
    println!("Sent: {}", message);

    let mut buf = [0; 1024];
    let bytes_received = socket.recv(&mut buf)?;
    let response = String::from_utf8_lossy(&buf[..bytes_received]);
    println!("Server replied: {}", response);

    Ok(())
}
