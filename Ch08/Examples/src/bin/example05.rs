fn main() {
    let s = "こんにちは";
    let start = s.char_indices().nth(2).map(|(i, _)| i).unwrap_or(s.len());
    let end = s.char_indices().nth(5).map(|(i, _)| i).unwrap_or(s.len());
    let slice = &s[start..end];
    println!("{}", slice);
}
