struct Book {
    title: String,
    author: String,
}

fn main() {
    let book = Book {
        title: String::from("Learning Rust"),
        author: String::from("Freek van Keulen"),
    };

    let moved_title = book.title;
    println!("Title: {}", moved_title);
    println!("Author: {}", book.author);
}