fn compare_and_display(a: impl std::fmt::Debug, b: impl std::fmt::Display) {
    println!("{:?}", a);
    println!("{}", b);
}

fn main() {
    compare_and_display((1, 2), 3);
}
