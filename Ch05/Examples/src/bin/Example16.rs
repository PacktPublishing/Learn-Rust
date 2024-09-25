fn identity<T: std::fmt::Debug>(x: T) -> T {
    x
}

fn main() {
    let s = "hello".to_string();
    println!("{}", identity(s));

    let arr: &[i32] = &[1, 2, 3];
    println!("{:?}", identity(arr));
}

