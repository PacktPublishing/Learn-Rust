struct LargeStruct {
    data: [u8; 1_000_000],
}

fn main() {
    let large = Box::new(LargeStruct {
        data: [0; 1_000_000],
    });

    println!("LargeStruct is on the heap, but the pointer is on the stack.");
    println!("The struct contains {} elements", large.data.len());
}