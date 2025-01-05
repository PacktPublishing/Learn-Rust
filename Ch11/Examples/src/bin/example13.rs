fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum); 

    numbers.iter().for_each(|x| println!("Number: {}", x));
}
