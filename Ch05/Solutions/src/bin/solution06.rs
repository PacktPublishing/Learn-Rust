fn largest<T: PartialOrd>(slice: &[T]) -> &T {
    let mut largest = &slice[0];

    for item in slice.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![101, 102, 201, 202, 103];
    let largest_number = largest(&numbers);
    println!("The largest number is: {}", largest_number);

    let chars = vec!['a', 'x', 'm', 'z', 'b'];
    let largest_char = largest(&chars);
    println!("The largest character is: {}", largest_char);
}
