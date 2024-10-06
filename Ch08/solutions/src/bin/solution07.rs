fn main() {
    let mut my_string = String::with_capacity(33);
    for _ in 0..100 {
        my_string.push('a');
        println!("Length: {}, Capacity: {}", my_string.len(), my_string.capacity());
    }
}
