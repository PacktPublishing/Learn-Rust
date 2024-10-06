fn main() {
    let input = "Hello, 世界!";
    print_chars_and_count(input);
}

fn print_chars_and_count(input: &str) {
    let mut count = 0;

    for ch in input.chars() {
        println!("{}", ch);
        count += 1;
    }

    println!("Total number of characters: {}", count);
    println!("Total number of bytes: {}", input.len());
}
