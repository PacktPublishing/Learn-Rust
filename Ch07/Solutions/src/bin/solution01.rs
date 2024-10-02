use std::f64;

fn square_root(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        Err(String::from("Cannot calculate the square root of a negative number"))
    } else {
        Ok(n.sqrt())
    }
}

fn main() {
    let positive_number = 25.0;
    let negative_number = -9.0;

    match square_root(positive_number) {
        Ok(result) => println!("Square root of {} is {}", positive_number, result),
        Err(e) => println!("Error: {}", e),
    }

    match square_root(negative_number) {
        Ok(result) => println!("Square root of {} is {}", negative_number, result),
        Err(e) => println!("Error: {}", e),
    }
}
