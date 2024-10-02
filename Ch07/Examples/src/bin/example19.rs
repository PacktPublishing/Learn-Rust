use std::fmt;

#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    ParseError,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(file) => write!(f, "File not found: {}", file),
            FileError::PermissionDenied(file) => write!(f, "Permission denied for file: {}", file),
            FileError::ParseError => write!(f, "Failed to parse file"),
        }
    }
}

impl std::error::Error for FileError {}

fn main() {
    let error = FileError::NotFound("config.toml".to_string());
    println!("{}", error);
}
