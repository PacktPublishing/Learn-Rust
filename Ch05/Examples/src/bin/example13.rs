fn print_and_clone(value: impl std::fmt::Display + Clone) {
    println!("{}", value);
    let _ = value.clone();
}
 fn main() {
     print_and_clone("Calling...")
 }