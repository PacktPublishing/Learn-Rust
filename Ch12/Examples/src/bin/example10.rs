use std::cell::Cell;

fn main() {
    let cell = Cell::new(5);
    println!("Initial value: {}", cell.get());

    cell.set(10);
    println!("Updated value: {}", cell.get());
}
