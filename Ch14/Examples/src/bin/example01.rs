fn get_words_from_title(title: &str) -> Vec<&str> {
    title.split_whitespace().collect()
}

fn main() {
    let book_title = String::from("Learning Rust");
    let words = get_words_from_title(&book_title);
    for word in words {
        println!("{}", word);
    }
}
