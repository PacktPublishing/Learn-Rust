fn main() {
    let number = 5;
    let message = if number > 0 {
        "Positive"
    }  else if number < 0 {
        "Negative"
    } else {
        "Zero"
    };
    println!("{message}");
}