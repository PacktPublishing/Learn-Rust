fn main() {
    let my_string = String::from("Nice to meet you");
    let length = my_string.len();
    let contains_substring = my_string.contains("you");
    println!("Length of the string: {}", length);
    println!("Does the string contain 'you'? {}", contains_substring);
}
