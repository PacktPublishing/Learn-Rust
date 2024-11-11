#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p1 = Point {
        x: 1,
        y: 2,
        z: 3,
    };
    println!("p1: {:?}", p1);

    let p2 = Point {
        x: 10,
        ..p1
    };
    println!("p2: {:?}", p2);
}
