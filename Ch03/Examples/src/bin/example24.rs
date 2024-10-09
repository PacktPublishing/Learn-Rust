fn main() {
    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("r1: {}, r2: {}", r1, r2);
}
