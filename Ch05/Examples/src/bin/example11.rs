fn largest<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let num = largest(10, 20);
    let char = largest('a', 'b');

    println!("Largest number: {}", num);
    println!("Largest char: {}", char);
}
