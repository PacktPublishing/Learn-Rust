fn main() {
    let mut numbers = vec![37; 5];
    numbers.push(24);
    numbers.push(25);
    numbers.pop();
    println!("{:?}", numbers);
}