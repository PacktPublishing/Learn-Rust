fn main() {
    let numbers = vec![1, 2, 3, 4];

    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    let product = numbers.into_iter().reduce(|acc, x| acc * x);
    println!("Product: {:?}", sum);
}
