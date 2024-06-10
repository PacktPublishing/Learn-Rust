fn main() {
    let mut x = 12;
    x = x + 2;
    {
        let y = x * 2;
        println!("y in inner scope: {}", y);
        println!("x from outer scope: {}", x);
    }
    println!("The value of x is: {}", x);
    println!("y is dropped and no longer accessible")
}
