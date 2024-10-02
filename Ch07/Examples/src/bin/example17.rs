fn process_file(file_name: &str) -> Result<(), String> {
    if file_name.is_empty() {
        return Err(String::from("Filename cannot be empty"));
    }
    // More logic here...
    Ok(())
}

fn main() {
    print!("{:?}", process_file(""));
}