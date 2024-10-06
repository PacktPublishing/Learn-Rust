fn main() {
    let input = "abc";
    match input.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing: {}", e),
    }
}