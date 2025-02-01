union MyUnion {
    int: i32,
    float: f32,
}

fn main() {
    let u = MyUnion { int: 10 };

    unsafe {
        println!("Union value: {}", u.float);
    }
}
