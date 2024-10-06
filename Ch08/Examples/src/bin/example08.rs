use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        let x = coords.next().unwrap().trim().parse()?;
        let y = coords.next().unwrap().trim().parse()?;
        Ok(Point { x, y })
    }
}

fn main() {
    let input = "10, 20";

    match Point::from_str(input) {
        Ok(point) => println!("Parsed point: {:?}", point),
        Err(e) => println!("Failed to parse point: {}", e),
    }
}