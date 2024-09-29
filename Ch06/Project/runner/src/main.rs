use geometric_shapes::shapes::{
    Shape,
    circle::Circle,
    square::Square,
    rectangle::Rectangle,
};

fn main() {
    let circle = Circle::new(3.0);
    let square = Square::new(4.0);
    let rectangle = Rectangle::new(4.0, 5.0);

    let shapes: [Box<dyn Shape>; 3] = [
        Box::new(circle),
        Box::new(square),
        Box::new(rectangle),
    ];

    let total_area: f64 = shapes.iter().map(|shape| shape.area()).sum();

    println!("The total area of all shapes is: {}", total_area);
}
