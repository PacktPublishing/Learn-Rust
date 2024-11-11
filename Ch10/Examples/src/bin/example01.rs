use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    println!("Hello, {}", input.trim());
}
