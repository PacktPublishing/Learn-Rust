fn apply_mut<F>(mut func: F, x: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    func(x)
}

fn main() {
    let mut factor = 2;
    let multiply = |x: i32| {
        factor *= 3;
        x * factor
    };

    println!("return value: {}", apply_mut(multiply, 3));
    println!("value of factor: {}", factor);
}