fn apply<F>(func: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    func(x)
}

fn main() {
    let y = 10;
    let add_y = |x: i32| x + y;
    println!("return value: {}", apply(add_y, 5));
    println!("value of y: {}", y);
}
