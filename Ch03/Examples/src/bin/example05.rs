fn main() {
    let mut a = 5;
    let b = &mut a;
    let c = &mut a;
    println!("{c}");
}
