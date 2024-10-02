fn is_even(num: i32) -> Result<i32, String> {
    if num % 2 == 0 {
        Ok(num)
    } else {
        Err("Not even".to_string())
    }
}

fn main() {
    let value: Result<i32, String> = Ok(4);
    let result = value.and_then(is_even);
    println!("{:?}", result);
}
