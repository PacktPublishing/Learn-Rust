#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&mut self, offset: &Point){
        self.x += offset.x;
        self.y += offset.y;
    }
}

fn main() {
    let mut p1 = Point{
        x: 2,
        y: 3,
    };
    let offset = Point{
        x: 5,
        y: 7,
    };
    p1.translate(&offset);
    println!("The point is: {:?}", p1);
}