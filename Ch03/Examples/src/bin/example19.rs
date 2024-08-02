fn main() {
    let numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three")];
    for num in numbers {
        println!("Immutable Borrowed value: {}", num);
    }
}