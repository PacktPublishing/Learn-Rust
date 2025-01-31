fn substring_after<'a>(text: &'a str, delimiter: &'a str) -> &'a str {
    text.split_once(delimiter).map(|(_, rest)| rest).unwrap_or("")
}

fn main() {
    println!("{}", substring_after("Learning Rust", " "));
}