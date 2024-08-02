// the function takes ownership of s, because here is no & used, so no reference
// effects:
// - s is not accessible after the function call in main
// - the value is dropped when the function ends

fn take_ownership(s: String) {
    println!("{s}");
}
fn main() {
    let s = String::from("test solution 2");
    take_ownership(s);
    // println!("{s}"); -> will not compile
}