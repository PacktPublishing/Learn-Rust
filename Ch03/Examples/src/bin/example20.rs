fn main() {
    let mut numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three")];
    for num in &mut numbers {
        num.push('*');
    }
    println!("{:?}", numbers);
}