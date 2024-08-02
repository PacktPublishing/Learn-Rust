fn main() {
    let mut numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    let taken = std::mem::replace(&mut numbers[1], String::new());
    println!("Taken value: {}", taken);
    println!("Array after taking an element: {:?}", numbers);
}
