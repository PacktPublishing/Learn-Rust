fn main() {
    let mut number = 10; // Initialize the variable

    number += 5;
    println!("After += 5, number is: {}", number);

    number -= 3;
    println!("After -= 3, number is: {}", number);

    number *= 2;
    println!("After *= 2, number is: {}", number);

    number /= 4;
    println!("After /= 4, number is: {}", number);

    number %= 3;
    println!("After %= 3, number is: {}", number);
}
