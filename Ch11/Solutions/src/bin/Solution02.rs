fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let mut iter = numbers.iter();

    println!("Iterating over vector elements:");

    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}