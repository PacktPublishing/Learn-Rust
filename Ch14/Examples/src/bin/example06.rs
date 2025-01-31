struct Borrowed<'a> {
    data: &'a str,
}

fn main() {
    let text = String::from("Hello, Rust!");
    let borrowed = Borrowed { data: &text };

    println!("{}", borrowed.data);
}
