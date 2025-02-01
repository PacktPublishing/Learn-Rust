fn main() {
    let mut x = 42;
    let raw_pointer: *mut i32 = &mut x;

    unsafe {
        let reference: &i32 = &*raw_pointer; // Bypassing borrow checker
        println!("{}", reference);
    }
}
