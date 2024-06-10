fn main() {
    let my_tuple = (2, "Hi there", '\u{1F980}');

    // Destructuring
    let (x, y, z) = my_tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Direct access
    let first_element = my_tuple.0;
}