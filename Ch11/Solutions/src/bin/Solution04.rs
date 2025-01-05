fn sum_of_even_numbers(numbers: &Vec<i32>) -> i32 {
    numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // Filter even numbers
        .sum() // Sum the even numbers
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum = sum_of_even_numbers(&numbers);
    println!("Sum of even numbers: {}", sum);
}
