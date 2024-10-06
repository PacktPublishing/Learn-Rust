fn main() {
    let input = "이것은 재미있는 예시입니다";
    let result = safe_slice(input);
    println!("{}", result);
}

fn safe_slice(input: &str) -> &str {
    let start = input.char_indices().nth(1).map(|(i, _)| i).unwrap_or(0);
    let end = input.char_indices().nth(4).map(|(i, _)| i).unwrap_or(input.len());
    &input[start..end]
}
