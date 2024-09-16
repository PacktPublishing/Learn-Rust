fn main() {
    'outer: for i in 1..=5 {
        for j in 1..=5 {
            if i == 3 && j == 3 {
                break 'outer; // Break out of the 'outer loop
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}