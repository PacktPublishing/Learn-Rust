fn main() {
    let s = "こんにちは, World!";
    let start = s.char_indices().nth(7).map(|(i, _)| i).unwrap_or(s.len());
    let end = s.char_indices().nth(13).map(|(i, _)| i).unwrap_or(s.len());
    let substring = &s[start..end];
    println!("Extracted substring: {}", substring);
    println!("Length in bytes: {}", s.len());
    println!("Number of characters: {}", s.chars().count());
}
