struct Color(u8, u8, u8);

fn main() {
    let color = Color(255, 0, 128);

    println!("Red: {}", color.0);
    println!("Green: {}", color.1);
    println!("Blue: {}", color.2);
}
