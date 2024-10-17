use std::collections::HashMap;

fn count_word_frequencies(text: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    word_count
}

fn main() {
    let text = "Rust is fast and memory safe and Rust is fun";
    let word_frequencies = count_word_frequencies(text);
    for (word, count) in word_frequencies {
        println!("{}: {}", word, count);
    }
}
