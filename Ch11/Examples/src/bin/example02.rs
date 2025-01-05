fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let func = square;
    println!("Square of 4: {}", func(4));
}
