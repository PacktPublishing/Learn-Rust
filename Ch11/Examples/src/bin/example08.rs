fn main() {
    let factor = 2;
    let multiply_by_factor = |x: i32| x * factor;
    println!("Multiply by factor: {}", multiply_by_factor(4));

    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    println!("Count: {}", count);

    let text = String::from("Hello");
    let consume_text = || println!("{}", text);
    consume_text();
}
