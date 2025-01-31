fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("Rust programming");
    let string2 = String::from("Memory safety");

    let result = longest(&string1, &string2);
    println!("The longer string is: {}", result);
}
