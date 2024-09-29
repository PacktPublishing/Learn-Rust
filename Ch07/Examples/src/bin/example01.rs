fn find_item(items: &[i32], target: i32) -> Option<i32> {
    for &item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    match find_item(&numbers, 3) {
        Some(number) => println!("Found: {}", number),
        None => println!("Item not found"),
    }

    match find_item(&numbers, 6) {
        Some(number) => println!("Found: {}", number),
        None => println!("Item not found"),
    }
}
