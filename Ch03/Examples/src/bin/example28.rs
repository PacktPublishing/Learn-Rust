fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    for i in 0..numbers.len() {
        numbers[0] *= numbers[i] * 2;
    }
    println!("{:?}", numbers);
}
