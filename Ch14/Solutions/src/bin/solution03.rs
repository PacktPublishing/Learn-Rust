fn substring_after(text: String, prefix: &str) -> &str {
    text.split_once(prefix).map(|(_, rest)| rest).unwrap_or("")
}

fn main() {
    let text = String::from("Learning Rust");
    let prefix = " ";
    let result = substring_after(text, prefix);

    println!("Substring after '{}': {}", prefix, result);
}
