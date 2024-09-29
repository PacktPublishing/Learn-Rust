use super::Shape;

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle::new(4.0, 5.0);
        assert_eq!(rectangle.area(), 20.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle::new(4.0, 5.0);
        assert_eq!(rectangle.perimeter(), 18.0);
    }
}
