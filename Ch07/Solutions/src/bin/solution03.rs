use std::f64;

fn square_root(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        Err(String::from("Cannot calculate the square root of a negative number"))
    } else {
        Ok(n.sqrt())
    }
}

fn safe_square_root(n: f64) -> Result<f64, String> {
    Ok(square_root(n)?)
}

fn main() {
    match safe_square_root(16.0) {
        Ok(result) => println!("Square root is: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_square_root(-4.0) {
        Ok(result) => println!("Square root is: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
