fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = numbers.into_iter()
        .filter(|x| x % 2 == 0) // Filter out odd numbers
        .map(|x| x * x) // Square the remaining numbers
        .collect(); // Collect into a new vector

    println!("Squared even numbers: {:?}", result);
}