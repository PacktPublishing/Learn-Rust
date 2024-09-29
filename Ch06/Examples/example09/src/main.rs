fn main() {
    println!("Hello, world!");
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        let result = divide(10, 2);
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_divide_failure() {
        let result = divide(10, 0);
        assert!(result.is_err());
    }
}
