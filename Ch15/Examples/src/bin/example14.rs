fn main() {
    let mut num = 10;
    let raw = &mut num as *mut i32;

    unsafe {
        *raw = 20;
        println!("num: {}", num);
    }
}
