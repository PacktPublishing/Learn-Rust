fn process_move_type(value: String) -> String {
    println!("Inside function: {}", value);
    value
}

fn main() {
    let s = String::from("Hello");
    let s_processed = process_move_type(s);
    println!("Processed value: {}", s_processed);
}
