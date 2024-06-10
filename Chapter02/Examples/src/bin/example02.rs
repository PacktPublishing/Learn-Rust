fn main() {
    let a: usize = 10;
    let b: isize = -5;
    // Cast `a` from `usize` to `isize` before adding
    let result: isize = a as isize + b;

    println!("The value of result is {result}");
}