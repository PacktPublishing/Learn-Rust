trait Draw {
    fn draw(&self);
}

trait Colored {
    fn color(&self) -> String;
}

trait ColoredShape: Draw + Colored {
    fn describe(&self) {
        println!("This shape is {} and can be drawn.", self.color());
    }
}

struct Circle;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }
}

impl Colored for Circle {
    fn color(&self) -> String {
        String::from("red")
    }
}

impl ColoredShape for Circle {}

fn main() {
    let circle = Circle;
    circle.draw();
    println!("Color: {}", circle.color());
    circle.describe();
}
