fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

fn main() {
    print_value(42);
    print_value(3.14);
    print_value("Hello!");
}
