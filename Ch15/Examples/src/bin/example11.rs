fn main() {
    let x = 42;
    let raw_const: *const i32 = &x;
    let _raw_mut: *mut i32 = &x as *const i32 as *mut i32;

    unsafe {
        let y = *raw_const;
        println!("y: {}", y);
    }
}
