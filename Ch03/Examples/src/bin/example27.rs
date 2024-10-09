fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    for num in &mut numbers {
        numbers[0] *= *num * 2;
    }
    println!("{:?}", numbers);
}