fn is_positive(n: i32) -> Result<i32, String> {
    if n < 0 {
        Err(String::from("The number is negative"))
    } else {
        Ok(n)
    }
}

fn is_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(String::from("The number is not even"))
    }
}

fn main() {
    let number = 4;

    let result = is_positive(number)
        .and_then(is_even);

    match result {
        Ok(n) => println!("The number {} is positive and even.", n),
        Err(e) => println!("Error: {}", e),
    }
}
