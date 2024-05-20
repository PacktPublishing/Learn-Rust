fn main() {
    let number = 6;
    let description = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    // Outputs: The number is even
    println!("The number is {}", description);
}