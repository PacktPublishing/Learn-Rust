use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "input04.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize the counter
    let mut count = 0;

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors
        if line.contains("Rust") {
            count += 1;
        }
    }

    // Print the result
    println!("The word 'Rust' appears in {} lines.", count);

    Ok(())
}
