enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,  // Area of a circle: πr²
            Shape::Rectangle(width, height) => width * height,                // Area of a rectangle: width * height
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);

    println!("The area of the circle is: {}", circle.area());
    println!("The area of the rectangle is: {}", rectangle.area());
}
