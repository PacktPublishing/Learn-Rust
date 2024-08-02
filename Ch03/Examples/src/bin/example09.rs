fn valid_reference(s: &String) -> &String {
    s
}

fn main() {
    let s = String::from("Hello, world!");
    let reference = valid_reference(&s);
    println!("{}", reference);
}
