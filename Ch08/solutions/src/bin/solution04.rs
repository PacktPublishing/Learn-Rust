fn main() {
    let mut s = String::from("Hi there");
    println!("Length in bytes: {}", s.len());
    println!("capacity in bytes: {}", s.capacity());

    s.push('!');
    println!("Length in bytes: {}", s.len());
    println!("capacity in bytes: {}", s.capacity());

    s = s.replace("Hi there", "Hello");
    println!("Length in bytes: {}", s.len());
    println!("capacity in bytes: {}", s.capacity());

    println!("{}", s);
}

