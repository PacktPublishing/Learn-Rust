fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let number1 = 4;
    let number2 = 7;

    println!("'{} is even' is {}", number1, is_even(number1));
    println!("'{} is even' is {}", number2, is_even(number2));
}
