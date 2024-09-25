#[derive(Clone)]
struct MyStruct {
    data: String,
}

fn main() {
    let s1 = MyStruct {
        data: String::from("Hello"),
    };
    // let s2 = s1;
    let s2 = s1.clone();
    println!("s1: {}", s1.data);
    println!("s2: {}", s2.data);
}
