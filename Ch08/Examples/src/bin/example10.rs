fn main() {
    let input = "12, 34, 56";
    for number in input.split(", ") {
        let number = number.parse::<u8>().unwrap();
        println!("number: {}", number);
    }
}
