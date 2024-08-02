fn main() {
    let mut fruit = [
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];

    let mut temp = String::from("no more fruit");
    std::mem::swap(&mut fruit[1], &mut temp);
    println!("Taken value: {}", temp);
    println!("Array after swapping: {:?}", fruit);
}
