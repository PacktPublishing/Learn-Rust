#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

fn main() {
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 4.2);

    println!("int_point: {:?}", int_point);
    println!("float_point: {:?}", float_point);
}
