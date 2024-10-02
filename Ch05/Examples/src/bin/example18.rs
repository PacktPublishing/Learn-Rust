fn make_vector<T>(item: T) -> Vec<T> {
    vec![item]
}

fn main() {
    let v = make_vector(77);
    println!("{:?}", v);
}