fn main() {
    let raw_pointer: *const i32 = &10;

    unsafe {
        println!("Value at raw pointer: {}", *raw_pointer);
    }
}
