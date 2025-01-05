fn apply_once<F>(func: F, x: i32) -> i32
where
    F: FnOnce(i32) -> i32,
{
    func(x)
}

fn main() {
    let factor = "5".to_string();
    let consume = move |x: i32| x * factor.parse::<i32>().unwrap();
    println!("return value: {}", apply_once(consume, 10));
    //println!("factor: {}", factor);
}
