fn process_copy_type(value: i32) -> i32 {
    println!("Inside function: {}", value);
    value * 2
}

fn main() {
    let x = 6;
    let y = process_copy_type(x);

    println!("Original value: {}", x);
    println!("Processed value: {}", y);
}
