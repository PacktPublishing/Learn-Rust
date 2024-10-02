fn factorial(n: i128) -> Result<i128, String> {
    if n < 0 {
        return Err("Factorial is not defined for negative numbers.".to_string());
    }
    if n <= 1 {
        return Ok(1);
    }

    match factorial(n - 1) {
        Ok(result) => {
            match result.checked_mul(n) {
                Some(factorial_value) => Ok(factorial_value),
                None => Err("Overflow occurred while calculating factorial.".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

fn main() {
    for value in -1..35 {
        match factorial(value) {
            Ok(result) => println!("Factorial of {} is {}", value, result),
            Err(err) => println!("Error calculating factorial of {}: {}", value, err),
        }
    }
}
