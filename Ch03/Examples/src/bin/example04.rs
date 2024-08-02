fn main() {
    let mut a = 4;
    let b = &mut a;
    *b += 1;
    println!("{b}");
}