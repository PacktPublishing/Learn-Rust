#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn translate(&self, offset: &Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

fn main() {}
