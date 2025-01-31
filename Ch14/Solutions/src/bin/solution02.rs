struct Word<'a> {
    text: &'a str,
}

fn create_word(text: &str) -> Word {
    Word { text }
}

fn main() {
    let single_word = String::from("Extraordinary");
    let word = create_word(&single_word);

    println!("Stored word: {}", word.text);
}
