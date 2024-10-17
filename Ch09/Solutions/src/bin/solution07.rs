use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::new();

    numbers.insert(10);
    numbers.insert(20);
    numbers.insert(30);
    numbers.insert(40);
    numbers.insert(50);

    let duplicate_insertion = numbers.insert(30);

    println!("HashSet after adding elements: {:?}", numbers);

    if !duplicate_insertion {
        println!("Attempt to insert duplicate element '30' was not allowed.");
    }
}
