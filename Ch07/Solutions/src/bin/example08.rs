fn get_valid_age(input: Option<i32>) -> Option<i32> {
    input
        .filter(|&age| age > 0)          // Ensure age is valid (positive)
        .or_else(|| Some(18))            // Provide default age of 18 if invalid or None
}

fn main() {
    let inputs = [Some(25), Some(-5), None, Some(0)];

    for input in inputs {
        let valid_age = get_valid_age(input);
        println!("Input: {:?}, Valid Age: {:?}", input, valid_age);
    }
}
