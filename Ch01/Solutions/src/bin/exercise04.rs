fn main() {
    let speed = 88;
    println!("The type of speed is: {}", std::any::type_name_of_val(&speed));
}