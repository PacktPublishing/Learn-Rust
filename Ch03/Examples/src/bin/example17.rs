fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6];
    let (first_part, second_part) = numbers.split_at_mut(2);
    let slice1 = &mut first_part[1..2];
    let slice2 = &mut second_part[0..1];
    println!("Slice1: {:?}", slice1);
    println!("Slice2: {:?}", slice2);
}
