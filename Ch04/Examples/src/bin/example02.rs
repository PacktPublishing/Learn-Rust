struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = Point {
        x: 3,
        y: 4,
    };
    println!("Point coordinates: ({}, {})", p.x, p.y);
}
