// You cannot use the array after the loop because it is consumed since & is not used.
fn main() {
    let numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    for number in numbers {
        println!("{number}");
    }

    // println!("{numbers:?}"); -> this will not compile
}