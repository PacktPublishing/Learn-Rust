#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(self, offset: &Point) -> Point {
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
        x: 10,
        y: 12,
    };
    let p1 = p1.translate(&offset);
    println!("The point is: {:?}", p1);
}
