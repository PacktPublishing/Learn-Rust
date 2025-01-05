use std::io;

fn main() {
    println!("Enter a list of numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let sum: i32 = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum();

    println!("The sum is: {}", sum);
}
