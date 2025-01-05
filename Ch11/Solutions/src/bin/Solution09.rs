fn filter_and_flatten(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    vecs.into_iter()
        .flat_map(|v| v.into_iter().filter(|&x| x % 2 == 0))
        .collect()
}

fn main() {
    let numbers = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];

    let result = filter_and_flatten(numbers);
    println!("Filtered and flattened vector: {:?}", result);
}
