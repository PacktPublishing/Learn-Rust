fn main() {
    let numbers = vec![10, -3, 14, -7, 5, -1, 22];

    for &number in &numbers {
        if number < 0 {
            continue;
        }

        println!("{}", number);
    }
}
