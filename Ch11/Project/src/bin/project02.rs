use std::collections::HashSet;

fn process_text(paragraph: &str) -> Vec<String> {
    let mut unique_words: HashSet<String> = paragraph
        .to_lowercase()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut sorted_words: Vec<String> = unique_words.drain().collect();
    sorted_words.sort();
    sorted_words
}

fn main() {
    let paragraph = "This is an example. Example texts are simple. SIMPLE but effective!";
    let result = process_text(paragraph);
    println!("Sorted unique words: {:?}", result);
}
