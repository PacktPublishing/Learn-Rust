fn main() {
    let number_str = "42";
    let number: i32 = number_str.parse::<i32>().unwrap();
    println!("Parsed number: {}", number);

    let number_str = "42";
    let number: i32 = number_str.parse::<i32>().expect("Unable to parse...");
    println!("Parsed number: {}", number);
}
