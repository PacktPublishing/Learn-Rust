use std::cell::Cell;

struct Point {
    x: Cell<i32>, // x is wrapped in Cell to allow mutation
    y: i32,
}

fn main() {
    let point = Point {
        x: Cell::new(10),
        y: 20,
    };

    point.x.set(15);
    println!("Updated x: {}", point.x.get());
    println!("y: {}", point.y);
}
