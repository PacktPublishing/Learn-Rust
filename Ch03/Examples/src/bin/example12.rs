struct Book {
    title: String,
    author: String,
}

fn main() {
    let book = Book {
        title: String::from("Learning Rust"),
        author: String::from("Freek van Keulen"),
    };

    let book_title = book.title;
    println!("Title: {}", book_title);
    println!("Author: {}", book.author);
}