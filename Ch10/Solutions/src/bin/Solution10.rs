use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(err) = file.read_to_string(&mut content) {
                println!("Failed to read the file '{}': {}", file_path, err);
            } else {
                println!("Contents of '{}':\n{}", file_path, content);
            }
        }
        Err(err) => {
            println!("Error opening file '{}': {}", file_path, err);
        }
    }
}

fn main() {
    // Test with an existing file
    let existing_file = "input10.txt";
    println!("Testing with existing file '{}':", existing_file);
    read_file(existing_file);

    // Test with a non-existent file
    let non_existent_file = "non_existent.txt";
    println!("\nTesting with non-existent file '{}':", non_existent_file);
    read_file(non_existent_file);
}
