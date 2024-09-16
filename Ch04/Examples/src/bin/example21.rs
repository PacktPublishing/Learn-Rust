fn find_value(slice: &[i32], target: i32) -> Option<usize> {
    for i in 0..slice.len() {
        if slice[i] == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    match find_value(&numbers, 3) {
        Some(index) => println!("Found at index {}.", index),
        None => println!(" not found in the slice."),
    }

    match find_value(&numbers, 6) {
        Some(index) => println!("Found at index {}.", index),
        None => println!("not found in the slice."),
    }
}
