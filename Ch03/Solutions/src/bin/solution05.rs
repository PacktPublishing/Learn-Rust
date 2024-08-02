fn append_text(s: &mut String) {
    s.push_str(", World!");
}

fn main() {
    let mut message = String::from("Hello");
    append_text(&mut message);
    println!("{message}");
}