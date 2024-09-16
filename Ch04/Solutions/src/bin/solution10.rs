struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn is_long(&self) -> bool {
        self.pages > 300
    }
}

fn main() {
    let original_book = Book {
        title: String::from("À la recherche du temps perdu"),
        author: String::from("Marcel Proust"),
        pages: 4200,
    };

    if original_book.is_long() {
        println!("'{}' is a long book!", original_book.title);
    } else {
        println!("'{}' is not a long book.", original_book.title);
    }

    let new_book = Book {
        title: String::from("Les Plaisirs et les Jours"),
        ..original_book
    };

    println!(
        "New book title: '{}', author: '{}', pages: {}",
        new_book.title, new_book.author, new_book.pages
    );
}
