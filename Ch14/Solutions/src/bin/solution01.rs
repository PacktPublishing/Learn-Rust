fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Hello Rust world!");
    let word = first_word(&sentence);
    println!("The first word is: {}", word);
}
