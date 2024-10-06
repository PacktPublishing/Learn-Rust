fn main() {
    let parsed_value = parse_str_to_int("42");
    println!("Parsed value: {}", parsed_value);

    let invalid_value = parse_str_to_int("abc");
    println!("Parsed value: {}", invalid_value);
}

fn parse_str_to_int(input: &str) -> i32 {
    input.parse().unwrap_or(0)
}
