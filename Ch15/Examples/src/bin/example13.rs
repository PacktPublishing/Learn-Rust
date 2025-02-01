fn main() {
    let raw_dangling: *const i32;
    {
        let x = 10;
        raw_dangling = &x;
    }

    println!("raw_dangling: {:?}", raw_dangling);
}