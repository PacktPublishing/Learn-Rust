use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true) // Open the file in append mode
        .create(true) // Create the file if it doesn't exist
        .open("example.bin")?;

    let data = [0xDE, 0xAD, 0xBE, 0xEF]; // Binary data to append
    file.write_all(&data)?; // Append the data to the file

    Ok(())
}
