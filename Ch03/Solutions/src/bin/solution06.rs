// The lifetime of b spans from its creation to its last use: from line 5 to line 7.
// The lifetime of c spans from line 6 to line 7.
// These lifetimes overlap, resulting in multiple references to the same data at the same time.

fn main() {
    let a = 6;
    let b = &a;
    let c = &a;
    print!("{a} {b} {c}");
}