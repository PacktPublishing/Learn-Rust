fn implicit_lifetimes(input: &str) -> &str {
    input
}

fn explicit_lifetimes<'a>(input: &'a str) -> &'a str {
    input
}

fn main() {
    // There is no difference in behaviour
    let input = String::from("Hello, Rust!");
    let implicit = implicit_lifetimes(&input);
    let explicit = explicit_lifetimes(&input);

    println!("Implicit: {}", implicit);
    println!("Explicit: {}", explicit);
}
