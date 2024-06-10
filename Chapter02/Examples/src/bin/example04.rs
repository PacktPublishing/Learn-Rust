fn main() {
    let x = 5;
    let y = { // The block is assigned to the variable y
        let z = x + 1;
        z + 2 // The value that the block evaluates to
    };
    println!("y: {}", y); // Prints "y: 8"
}