// the function takes ownership of n, because here is no & used, so no reference
// Since the type is `Copy`, a copy of the value is passed to the function.
// effects:
// - n is accessible after the function call in main
// - p is dropped when the function take_ownership ends
// - n is dropped when the main function ends

fn take_ownership(p: i32) {
    println!("{p}");
}
fn main() {
    let n = 3;
    take_ownership(n);
    println!("{n}");
}