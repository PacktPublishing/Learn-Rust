fn main() {
    let numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three")];
    for num in numbers {
        println!("Moved value: {}", num);
    }
}