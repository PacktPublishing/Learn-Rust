use std::fs::File;
use std::io::{self, Read};

fn read_file_content(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(error) => println!("Failed to read file: {}", error),
    }
}
