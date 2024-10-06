#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let s = format!("{:?}", p);
    println!("Debug format: {:}", s);
}
