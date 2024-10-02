fn print_value<T: ?Sized + std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}


fn main() {
    let arr: &[i32] = &[1, 2, 3];
    print_value(arr);
}
