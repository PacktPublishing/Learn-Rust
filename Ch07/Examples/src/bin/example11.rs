fn main() {
    let some_number = Some(10);
    let none_number: Option<i32> = None;

    let result1 = some_number.or_else(|| Some(20));
    let result2 = none_number.or_else(|| Some(20));

    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);

    let result3 = none_number.or_else(|| None);
    println!("Result 3: {:?}", result3);
}
