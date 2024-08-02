fn main() {
    let number = 5;

    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("The number {} is {}", number, result);
}