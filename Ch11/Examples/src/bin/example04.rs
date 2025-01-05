fn create_multiplier(multiplier: i32) -> impl Fn(i32) -> i32 {
    move |x| x * multiplier
}

fn main() {
    let multiply_by_3 = create_multiplier(3);
    println!("3 x 5 = {}", multiply_by_3(5));
}
