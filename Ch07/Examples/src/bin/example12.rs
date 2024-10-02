fn main() {
    let some_number = Some(10);
    let none_number: Option<i32> = None;

    let even_number = some_number.filter(|&n| n % 2 == 0);
    let odd_number = some_number.filter(|&n| n % 2 != 0);

    println!("Even number: {:?}", even_number); // Output: Some(10)
    println!("Odd number: {:?}", odd_number);   // Output: None

    let filtered_none = none_number.filter(|&n| n % 2 == 0);
    println!("Filtered None: {:?}", filtered_none); // Output: None
}
