use std::f64;
use crate::MathError::UnsupportedOperation;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
    UnsupportedOperation,
}

fn perform_math_operation(a: f64, b: f64, operation: &str) -> Result<f64, MathError> {
    match operation {
        "divide" => {
            if b == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        "log" => {
            if a <= 0.0 {
                Err(MathError::NegativeLogarithm)
            } else {
                Ok(a.log(b))
            }
        }
        "sqrt" => {
            if a < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(a.sqrt())
            }
        }
        _ => Err(UnsupportedOperation),
    }
}

fn main() {
    let operation = "divide";
    match perform_math_operation(10.0, 0.0, operation) {
        Ok(result) => println!("Result of {} is: {}", operation, result),
        Err(MathError::DivisionByZero) => println!("Error: Division by zero is not allowed."),
        Err(MathError::NegativeLogarithm) => println!("Error: Logarithm of a non-positive number."),
        Err(MathError::NegativeSquareRoot) => println!("Error: Square root of a negative number."),
        Err(UnsupportedOperation) => println!("Error: Unknown operation"),
    }

    let operation = "sqrt";
    match perform_math_operation(-9.0, 0.0, operation) {
        Ok(result) => println!("Result of {} is: {}", operation, result),
        Err(MathError::DivisionByZero) => println!("Error: Division by zero is not allowed."),
        Err(MathError::NegativeLogarithm) => println!("Error: Logarithm of a non-positive number."),
        Err(MathError::NegativeSquareRoot) => println!("Error: Square root of a negative number."),
        Err(UnsupportedOperation) => println!("Error: Unknown operation"),
    }
}
