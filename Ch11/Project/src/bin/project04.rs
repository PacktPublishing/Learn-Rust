fn process_nested_data(data: Vec<Vec<Option<i32>>>, scale: i32) -> Vec<i32> {
    let mut result: Vec<i32> = data
        .into_iter()
        .flat_map(|inner| inner.into_iter().filter_map(|x| x))
        .filter(|&x| x >= 0)
        .map(|x| x * scale)
        .collect();

    result.sort();
    result.dedup();
    result
}

fn main() {
    let nested_data = vec![
        vec![Some(1), Some(-2), None, Some(3)],
        vec![Some(4), None, Some(-5), Some(3)],
        vec![Some(2), Some(6), None, Some(4)],
    ];

    let scaled_result = process_nested_data(nested_data, 2);
    println!("Processed and scaled data: {:?}", scaled_result);
}
