fn get_positive_number(input: &str) -> i32 {
    input
        .parse::<i32>()
        .map(|n| if n < 0 {0} else {n})
        .unwrap_or(0)
}

fn main() {
    println!("Input: '5', Output: {}", get_positive_number("5"));
    println!("Input: '-3', Output: {}", get_positive_number("-3"));
    println!("Input: 'abc', Output: {}", get_positive_number("abc"));
    println!("Input: '0', Output: {}", get_positive_number("0"));
    println!("Input: '-100', Output: {}", get_positive_number("-100"));
}
