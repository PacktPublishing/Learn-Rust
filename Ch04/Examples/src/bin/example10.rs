#[derive(Debug)]
struct Container {
    a: String,
    b: String,
}

fn main() {
    let c1 = Container {
        a: String::from("Hello"),
        b: String::from("Rust")
    };
    let _a_string = c1.a;

    println!("{}", c1.b);
}