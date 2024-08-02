// note: the length of this string is 24 and not 22, the number of characters
// more on this in the chapter about strings

fn print_lenght(s: &String) {
    println!("{}", s.len());
}

fn main() {
    let text = String::from("Registered Trademark ™");
    print_lenght(&text)
}