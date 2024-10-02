fn check_positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err("Not positive".to_string()) }
}

fn is_even(num: i32) -> Result<i32, String> {
    if num % 2 == 0 { Ok(num) } else { Err("Not even".to_string()) }
}

fn main() {
    let value = Ok(10)
        .and_then(check_positive)
        .and_then(is_even)
        .map(|x| x * 2)
        .or(Err("Some check failed"));

    println!("{:?}", value);
}