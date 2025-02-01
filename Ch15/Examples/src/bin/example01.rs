fn main() {
    let x = 42;
    let raw_pointer: *const i32 = &x;

    unsafe {
        println!("Value at raw pointer: {}", *raw_pointer);
    }
}