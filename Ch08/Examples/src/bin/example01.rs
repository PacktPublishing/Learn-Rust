fn main() {
    let mut s = String::new();
    println!("Length: {}", s.len());

    s.push_str("Hello, Rust!");
    println!("Content: {}", s);
    println!("Length: {}", s.len());
}
