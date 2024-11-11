#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let current_direction = Direction::South;

    println!("Current direction: {:?}", current_direction);
}