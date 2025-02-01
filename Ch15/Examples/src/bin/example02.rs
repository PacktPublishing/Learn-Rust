unsafe fn unsafe_function() {
    println!("This is an unsafe function");
}

fn main() {
    unsafe {
        unsafe_function();
    }
}
