use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";
    let contents = fs::read(file_path)?;
    println!("File contents (bytes): {:?}", contents);
    Ok(())
}
