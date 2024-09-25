trait Describable {
    fn describe(&self) -> String {
        "This is an item".to_string()
    }
}

struct Book;

impl Describable for Book {
    fn describe(&self) -> String {
        "This is a book".to_string()
    }
}

fn main() {
    let book = Book;

    println!("{}", book.describe());
}