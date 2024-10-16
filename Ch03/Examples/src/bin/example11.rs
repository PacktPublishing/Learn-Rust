fn valid_mutable_reference(s: &mut String) -> &String {
    s.push_str(", world!");
    s
}

fn main() {
    let mut s = String::from("Hello");
    println!("{}", s);
    let result = valid_mutable_reference(&mut s);
    println!("{}", s);
    println!("{}", result);
}
