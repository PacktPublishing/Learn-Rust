fn main() {
    let fibonacci: [usize; 7] = [0, 1, 1, 2, 3, 5, 8];
    let slice = &fibonacci[2..5];
    println!("{:?}", slice);
}
