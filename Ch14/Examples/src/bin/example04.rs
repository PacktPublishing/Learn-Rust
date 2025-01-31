fn substring_after<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.split_once(prefix).map(|(_, rest)| rest).unwrap_or("")
}

fn main() {
    let result;
    let text = String::from("Learning Rust");
    {
        let prefix = String::from(" ");
        result = substring_after(&text, &prefix);
    }
    println!("{}", result);
}
