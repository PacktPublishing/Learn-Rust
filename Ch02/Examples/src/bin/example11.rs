fn main() {
    let number = 2;
    match number {
        // 1 | 2 | 3 matches if number is 1, 2, or 3
        1 | 2 | 3 => println!("One, two, or three"),
        4 | 5 | 6 => println!("Four, five, or six"),
        _ => println!("Some other number"),
    }
}