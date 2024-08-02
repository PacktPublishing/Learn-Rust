fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6, 7];
    let slice = &mut numbers[2..5];
    slice[0] = 101;
    println!("{numbers:?}");
}