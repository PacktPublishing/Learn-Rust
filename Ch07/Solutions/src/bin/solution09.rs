use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
struct DataFetchError;

impl std::fmt::Display for DataFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to fetch data")
    }
}

impl Error for DataFetchError {}

fn fetch_data(api_data: &str) -> Result<&str, Box<dyn Error>> {
    if api_data.is_empty() {
        Err(Box::new(DataFetchError))
    } else {
        Ok(api_data)
    }
}

fn parse_data(data: &str) -> Result<i32, ParseIntError> {
    data.parse::<i32>()
}

fn fetch_and_parse_data(api_data: &str) -> Result<i32, Box<dyn Error>> {
    let data = fetch_data(api_data)?;
    let number = parse_data(data)?;
    Ok(number)
}

fn main() {
    match fetch_and_parse_data("42") {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_and_parse_data("") {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_and_parse_data("invalid_data") {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}
