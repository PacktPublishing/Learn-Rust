fn main() {
    let fibonacci: [usize; 8] = [0, 1, 1, 2, 3, 5, 8, 13];
    let slice = &fibonacci[2..5];
    println!("{:?}", slice);
}
