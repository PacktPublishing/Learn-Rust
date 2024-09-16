use std::io;

fn main() {
    println!("Please enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input: i32 = input.trim().parse().unwrap();
    println!("You entered: {}", input);
}
