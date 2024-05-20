fn main() {
    let age = 30;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        65..=120 => println!("Senior"),
        _ => println!("Invalid age"),
    }
}
