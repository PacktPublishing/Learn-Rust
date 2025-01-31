// You cannot return 'second' here because it has a different and potentially shorter lifetime
fn compare_lengths<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let string1 = String::from("Longer string");
    let result;
    {
        let temp_string = String::from("Short");
        result = compare_lengths(&string1, &temp_string);
        println!("Longer string is: {}", result);
    }
}