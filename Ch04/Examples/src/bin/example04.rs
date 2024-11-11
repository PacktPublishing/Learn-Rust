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

fn main() {
    let p1 = Point{
        x: 2,
        y: 3,
    };
    let offset = Point{
        x: 4,
        y: 5,
    };
    let translated_point = p1.translate(&offset);
    println!("The point is: {:?}", translated_point);
}