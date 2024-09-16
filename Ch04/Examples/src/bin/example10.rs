#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

fn main() {
    let mut p1 = Point::new(2, 3);
}