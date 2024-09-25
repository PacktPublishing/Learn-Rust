trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle with radius: {}", self.radius);
    }
}

#[derive(Copy, Clone)]
struct Square {
    side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a Square with side length: {}", self.side);
    }
}

fn draw_shapes(shapes: &[&dyn Drawable]) {
    for shape in shapes {
        shape.draw();
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };

    let shapes: [&dyn Drawable; 2]= [&circle, &square];

    draw_shapes(&shapes);
}