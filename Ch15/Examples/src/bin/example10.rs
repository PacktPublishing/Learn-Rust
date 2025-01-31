fn main() {
    let mut value: i32 = 42;

    let immutable_ptr: *const i32 = &value; // Immutable raw pointer
    let mutable_ptr: *mut i32 = &mut value as *const i32 as *mut i32; // Mutable raw pointer

    unsafe {
        *mutable_ptr = 43;
    }

    println!("Immutable pointer address: {:?}", immutable_ptr);
    println!("Mutable pointer address: {:?}", mutable_ptr);

    println!("value: {:?}", value);
}
