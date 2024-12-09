use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "input07.txt";

    // Get the file metadata
    let metadata = fs::symlink_metadata(file_path)?;

    // Determine the file type
    if metadata.is_file() {
        println!("The path '{}' is a regular file.", file_path);
    } else if metadata.is_dir() {
        println!("The path '{}' is a directory.", file_path);
    } else if metadata.file_type().is_symlink() {
        println!("The path '{}' is a symbolic link.", file_path);
    } else {
        println!("The path '{}' is of an unknown type.", file_path);
    }

    // Print the file size
    println!("Size: {} bytes", metadata.len());

    Ok(())
}
