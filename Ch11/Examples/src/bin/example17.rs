fn main() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];

    let flattened: Vec<_> = nested.into_iter().flat_map(|v| v).collect();
    println!("Flattened: {:?}", flattened);
}
