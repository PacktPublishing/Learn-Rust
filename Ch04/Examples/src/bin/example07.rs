#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

fn main() {
    let mut _p1 = Point::new(2, 3);
}