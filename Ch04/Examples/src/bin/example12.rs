struct Point {
    x: i32,
    y: i32,
}

fn move_point(point: &mut Point, dx: i32, dy: i32) {
    point.x += dx;
    point.y += dy;
}

fn main() {
    let mut p = Point { x: 5, y: 10 };
    move_point(&mut p, 3, 4);
}