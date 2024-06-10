fn main() {
    let number = 3;
    let message = match number {
        1 => "uno",
        2 => "due",
        3 => "tre",
        _ => "qualcos'altro",
    };
    println!("message: {message}");
}