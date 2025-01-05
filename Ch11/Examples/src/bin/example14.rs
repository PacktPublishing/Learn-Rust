fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = numbers
        .iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .take(2)
        .collect();

    println!("Result: {:?}", result);
}
