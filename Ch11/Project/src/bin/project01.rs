fn remove_negatives_and_increment(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter()
        .filter(|&x| x >= 0)  // Remove negative numbers
        .map(|x| x + 1)      // Increment remaining numbers by 1
        .collect()           // Collect into a new list
}

fn main() {
    let numbers = vec![-3, -1, 0, 2, 5, -7, 8];
    let result = remove_negatives_and_increment(numbers);
    println!("Processed list: {:?}", result);
}
