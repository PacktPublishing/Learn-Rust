trait Shape {
    fn area(&self) -> f64;
}

trait Colored: Shape {
    fn color(&self) -> String;
}

struct Square {
    side: f64,
    color: String,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Colored for Square {
    fn color(&self) -> String {
        self.color.clone()
    }
}

fn main() {
    let square = Square { side: 3.0, color: String::from("red") };

    println!("Square area: {}", square.area());
    println!("Square color: {}", square.color());
}