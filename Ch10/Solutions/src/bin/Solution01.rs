use std::io::{self, Write};

fn main() {
    // Create a mutable string to store the name
    let mut name = String::new();
    print!("Please enter your name: ");
    io::stdout().flush().ok();

    // Read the user's name
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim(); // Remove any trailing newline or whitespace

    // Create a mutable string to store the age
    let mut age = String::new();
    print!("Please enter your age: ");
    io::stdout().flush().ok();

    // Read the user's age
    io::stdin().read_line(&mut age).expect("Failed to read input");

    // Parse the age as an integer
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    // Print the personalized message
    println!("Hello, {}! You are {} years old.", name, age);
}