fn main() {
    let n = 5;
    let result = math::factorial(n);
    println!("The factorial of {} is {}", n, result);
}

mod math {
    pub fn factorial(n: u32) -> u128 {
        (1..=n as u128).product()
    }
}
