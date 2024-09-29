pub mod circle;
pub mod square;
pub mod rectangle;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
