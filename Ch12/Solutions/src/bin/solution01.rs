use std::mem;

struct LargeStruct {
    data: [i32; 100_000],
}

fn main() {
    let large_struct = Box::new(LargeStruct {
        data: [0; 100_000],
    });

    println!("The size of the array is: {}", large_struct.data.len());

    println!(
        "The size of Box<LargeStruct> in memory is: {} bytes",
        mem::size_of_val(&large_struct)
    );

    println!(
        "The size of LargeStruct in memory is: {} bytes",
        mem::size_of::<LargeStruct>()
    );
}