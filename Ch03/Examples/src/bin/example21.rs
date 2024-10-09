fn main() {
    let mut numbers = [1, 2, 3];
    for num in &mut numbers {
        *num += 1;
    }
    println!("{:?}", numbers);
}
