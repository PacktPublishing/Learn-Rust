#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p1 = Point::new(2, 3);
    let p2 = p1.clone();

    println!("p1: {:#?}", p1);
    println!("p2: {:#?}", p2);

    println!("{}", p1 == p2);
}