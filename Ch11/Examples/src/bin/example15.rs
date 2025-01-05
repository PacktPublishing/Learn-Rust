fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
}
