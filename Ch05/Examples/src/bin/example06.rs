#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let point1 = Point { x: 10, y: 20 };
    let point2 = point1;

    println!("Point1: {:?}", point1);
    println!("Point2: {:?}", point2);

    let point3 = point1.clone();
    println!("Point3: {:?}", point3);
}
