use super::Shape;

pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_area() {
        let square = Square::new(4.0);
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_square_perimeter() {
        let square = Square::new(4.0);
        assert_eq!(square.perimeter(), 16.0);
    }
}
