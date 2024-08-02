fn main() {
    let number = 10;
    match number {
        n if n % 2 == 0 => println!("Even"),
        n if n % 2 != 0 => println!("Odd"),
        _ => println!("Unknown"),
    }
}