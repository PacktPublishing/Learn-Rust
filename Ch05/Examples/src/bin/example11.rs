fn display_and_clone<T: std::fmt::Display + Clone>(value: T) {
    println!("{}", value);
    let _copy = value.clone();
}

fn main() {
    let s = String::from("Hello, world!");

    display_and_clone(s); 
}