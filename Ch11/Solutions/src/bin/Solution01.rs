fn transform_list<F>(list: &mut Vec<i32>, transform: F)
where
    F: Fn(i32) -> i32,
{
    for i in list.iter_mut() {
        *i = transform(*i);
    }
}

fn square(x: i32) -> i32 {
    x * x
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    println!("Original list: {:?}", numbers);

    // Apply square transformation
    transform_list(&mut numbers, square);
    println!("After squaring: {:?}", numbers);

    // Apply doubling transformation
    transform_list(&mut numbers, double);
    println!("After doubling: {:?}", numbers);
}