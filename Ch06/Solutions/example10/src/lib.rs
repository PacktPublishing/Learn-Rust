fn fibonacci(n: i32) -> Result<u64, String> {
    if n < 0 {
        return Err("The index cannot be negative.".to_string());
    }

    let n = n as u64;
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }

    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_success() {
        assert_eq!(fibonacci(0), Ok(0));
        assert_eq!(fibonacci(1), Ok(1));
        assert_eq!(fibonacci(5), Ok(5));
        assert_eq!(fibonacci(10), Ok(55));
    }

    #[test]
    fn test_fibonacci_failure() {
        assert_eq!(fibonacci(-1), Err("The index cannot be negative.".to_string()));
        assert_eq!(fibonacci(-10), Err("The index cannot be negative.".to_string()));
    }
}
