use std::collections::HashMap;

fn word_frequency(input: &str) -> HashMap<String, usize> {
    input
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}

fn main() {
    let text = "hello world hello rust rust rust";
    let frequencies = word_frequency(text);

    for (word, count) in frequencies {
        println!("{}: {}", word, count);
    }
}
