mod store {
    pub struct Book {
        pub title: String,
        pub price: f64,
        pages: u32,
    }

    impl Book {
        pub fn new(title: String, price: f64, pages: u32) -> Self {
            Self { title, price, pages }
        }
    }
}

fn main() {
    let my_book = store::Book::new(
        String::from("Rust Programming"),
        29.99, 300);

    println!("{} {}", my_book.title, my_book.price);
}
