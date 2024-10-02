fn find_even(items: &[i32]) -> Option<i32> {
    for &item in items {
        if item % 2 == 0 {
            return Some(item);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 7, 10, 11];

    match find_even(&numbers) {
        Some(even) => println!("Found an even number: {}", even),
        None => println!("No even numbers found"),
    }

    let more_numbers = [1, 3, 5, 7, 9];

    match find_even(&more_numbers) {
        Some(even) => println!("Found an even number: {}", even),
        None => println!("No even numbers found"),
    }
}
