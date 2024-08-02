// You can use the array after the loop because it is uses references.
fn main() {
    let numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    for number in &numbers {
        println!("{number}");
    }

    println!("{numbers:?}"); // now this will compile
}