fn find_long_string(strings: Vec<String>) -> Result<String, String> {
    strings.into_iter()
        .find(|s| s.len() > 5)
        .ok_or("No string longer than 5 characters found".to_string())
}

fn main() {
    let strings = vec![
        "cat".to_string(),
        "elephant".to_string(),
        "dog".to_string(),
        "bird".to_string(),
        "dolphin".to_string(),
    ];

    match find_long_string(strings) {
        Ok(result) => println!("Found: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
