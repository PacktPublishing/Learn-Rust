fn main() {
    println!("Hello, world!");
}

fn find_item(items: Vec<i32>, target: i32) -> Option<i32> {
    for item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_item_some() {
        let items = vec![1, 2, 3, 4];
        assert_eq!(find_item(items, 3), Some(3));
    }

    #[test]
    fn test_find_item_none() {
        let items = vec![1, 2, 3, 4];
        assert_eq!(find_item(items, 5), None);
    }
}
