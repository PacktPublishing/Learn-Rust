fn main() {
    let result = format_name_and_age("Alice", 30);
    println!("{}", result);
}

fn format_name_and_age(name: &str, age: u32) -> String {
    format!("{name} is {age} years old.")
}
