struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: &Point) {
    println!("Point is at ({}, {})", point.x, point.y);
}

fn main() {
    let p = Point { x: 5, y: 10 };
    print_point(&p);
}
