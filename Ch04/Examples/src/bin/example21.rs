#[derive(Debug)]
enum SqrtError {
    NegativeNumber,
}

fn calculate_sqrt(number: f64) -> Result<f64, SqrtError> {
    if number < 0.0 {
        Err(SqrtError::NegativeNumber)
    } else {
        Ok(number.sqrt())
    }
}

fn main() {
    let numbers = vec![4.0, -3.0, 9.0, -1.0];

    for number in numbers {
        match calculate_sqrt(number) {
            Ok(sqrt) => println!("The square root of {} is {}.", number, sqrt),
            Err(e) => println!("Error: {:?} for number {}", e, number),
        }
    }
}
