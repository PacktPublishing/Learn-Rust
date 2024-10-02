fn complex_function<T, U>(t: T, u: U)
where
    T: std::fmt::Debug + Clone,
    U: std::fmt::Display + PartialEq,
{
    println!("{:?}", t);
    let _clone = t.clone();
    if u == u {
        println!("{}", u);
    }
}

fn print_value(value: impl std::fmt::Display) {
    println!("{}", value);
}


fn main() {
    let num = 42;
    let text = "Hello";

    complex_function(num, text);
}
