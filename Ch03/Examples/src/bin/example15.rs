fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);
    println!("Original array: {:?}", numbers);
}