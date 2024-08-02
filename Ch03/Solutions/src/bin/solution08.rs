fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let (slice1, slice2) = numbers.split_at_mut(4);
    slice1[0] = 100;
    slice2[0] = 101;
    println!("{numbers:?}");
}