fn main() {
    let numbers = vec![1, 2, 3];
    let mut iter = numbers.iter();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
