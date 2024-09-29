use super::Shape;

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(3.0);
        assert!((circle.area() - 28.274).abs() < 0.001);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(3.0);
        assert!((circle.perimeter() - 18.849).abs() < 0.001);
    }
}
