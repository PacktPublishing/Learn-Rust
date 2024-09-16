fn main() {
    for number in 1..10 {
        if number % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", number);
    }
}