fn main() {
    let number = 5;

    let factorial = {
        let mut result = 1;
        for i in 1..=number {
            result *= i;
        }
        result
    };

    println!("The factorial of {} is {}", number, factorial);
}
