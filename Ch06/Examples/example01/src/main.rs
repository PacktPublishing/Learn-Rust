mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        internal_add(a, b)
    }

    fn internal_add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("The sum is: {}", math::add(5, 3));
}
