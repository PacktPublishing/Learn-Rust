fn main() {
    for number in 1..10 {
        if number % 2 == 0 {
            continue; // start next iteration
        }
        println!("Odd number: {}", number);
    }
}