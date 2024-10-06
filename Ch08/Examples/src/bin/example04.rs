fn main() {
    let s = "Bonjour".to_owned();
    println!("{}", s);

    let s: String = "Bonjour".into();
    println!("{}", s);

    let slice = &s[0..3];
}
