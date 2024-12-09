use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    let file = File::open("example.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 16];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read])
    }
    Ok(())
}
