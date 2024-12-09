use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:7878")?;
    println!("UDP server listening on 127.0.0.1:7878");

    let mut buf = [0; 1024];
    loop {
        let (bytes_received, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..bytes_received]);
        println!("Received '{}' from {}", msg, src);

        let response = format!("Hello, {}!", src);
        socket.send_to(response.as_bytes(), src)?;
    }
}