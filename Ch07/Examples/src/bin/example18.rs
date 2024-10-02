#[derive(Debug)]
enum FileError {
    EmptyFilename,
    NotFound(String),
    ParseError,
}

fn process_file(file_name: &str) -> Result<(), FileError> {
    if file_name.is_empty() {
        return Err(FileError::EmptyFilename);
    }
    // More logic here...
    Ok(())
}

fn main() {
    println!("{:?}", process_file(""));
}