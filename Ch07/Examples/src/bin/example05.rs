fn main() {
    let number: Option<i32> = Some(5);
    println!("The number is: {}", number.unwrap());

    let result: Result<&str, &str> = Ok("Success!");
    println!("Result: {}", result.unwrap());
}