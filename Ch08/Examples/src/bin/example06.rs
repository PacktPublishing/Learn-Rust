fn main() {
    let s = "Hello, world!";
    if let Some(slice) = s.get(7..12) {
        println!("{}", slice);
    } else {
        println!("Invalid slice");
    }
}