fn parse_strings_to_integers(strings: Vec<String>) -> Result<Vec<i32>, Vec<String>> {
    let (parsed, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>().map_err(|_| s.clone()))
        .partition(Result::is_ok);

    if errors.is_empty() {
        Ok(parsed.into_iter().map(Result::unwrap).collect())
    } else {
        Err(errors.into_iter().map(Result::unwrap_err).collect())
    }
}

fn main() {
    let strings = vec!["10".to_string(), "20a".to_string(), "30".to_string()];
    match parse_strings_to_integers(strings) {
        Ok(integers) => println!("Parsed integers: {:?}", integers),
        Err(errors) => println!("Failed to parse: {:?}", errors),
    }
}
