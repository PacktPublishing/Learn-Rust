fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    s.push_str(", world!");
    println!("{}", r1);
}
