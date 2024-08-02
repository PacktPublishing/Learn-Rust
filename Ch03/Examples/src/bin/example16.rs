fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6];
    let slice1 = &mut numbers[1..2];
    let slice2 = &mut numbers[3..5];
    println!("Slice: {:?}", slice1);
}