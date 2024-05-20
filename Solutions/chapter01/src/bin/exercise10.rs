fn main() {
    let a = 1234;

    {
        let a = a * 3;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a in the inner scope is: {a}");
}