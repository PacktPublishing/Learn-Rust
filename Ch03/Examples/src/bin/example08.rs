fn print_value(value: &String) {
    println!("Value inside function: {}", value);
}

fn main() {
    let large_string = String::from("Hello again");

    print_value(&large_string);
    println!("Original value: {}", large_string);
}
