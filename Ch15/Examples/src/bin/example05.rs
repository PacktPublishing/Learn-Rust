unsafe fn dangerous_function() {
    println!("Performing an unsafe operation!");
}

fn main() {
    unsafe {
        dangerous_function();
    }
}
