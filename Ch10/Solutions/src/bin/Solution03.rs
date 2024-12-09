use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file for reading
    let file_path = "input03.txt";
    let mut file = File::open(file_path)?;

    // Buffer to hold the chunks
    let mut buffer = [0; 16];

    loop {
        // Read up to 16 bytes into the buffer
        let bytes_read = file.read(&mut buffer)?;

        // Break the loop if the end of the file is reached
        if bytes_read == 0 {
            break;
        }

        // Convert the buffer to a string slice and print it
        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("{}", chunk);
    }

    Ok(())
}
