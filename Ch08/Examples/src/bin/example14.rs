fn main() {
    let mut inefficient = String::new();
    for i in 0..5 {
        inefficient.push_str("hello ");
    }
    println!("Inefficient String: {}", inefficient);

    let mut efficient = String::with_capacity(30);
    for _ in 0..5 {
        efficient.push_str("hello ");
    }
    println!("Efficient String: {}", efficient);
}
