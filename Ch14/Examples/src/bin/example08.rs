fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap()
}

fn main() {
    println!("{}", first_word("Hello, world!"));
}