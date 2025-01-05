fn apply_function<F>(value: i32, func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = apply_function(5, double);
    println!("Result: {}", result);
}
