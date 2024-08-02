fn main() {
    let value: i32;
    if false {
        value = 10;
    }
    // compiler error
    println!("The delayed value is: {}", value);
}
