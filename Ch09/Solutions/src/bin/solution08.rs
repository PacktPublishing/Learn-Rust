use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl Eq for Book {}

impl Hash for Book {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.author.hash(state);
    }
}

fn main() {
    let mut book_store: HashMap<Book, u32> = HashMap::new();

    book_store.insert(
        Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
        },
        10,
    );

    book_store.insert(
        Book {
            title: "Rust for Beginners".to_string(),
            author: "John Doe".to_string(),
        },
        5,
    );

    book_store.insert(
        Book {
            title: "Advanced Rust Programming".to_string(),
            author: "Jane Smith".to_string(),
        },
        3,
    );

    let search_book = Book {
        title: "Rust for Beginners".to_string(),
        author: "John Doe".to_string(),
    };

    match find_copies(&book_store, search_book) {
        Some(copies) => println!("Copies available: {}", copies),
        None => println!("Book not found in the store."),
    }
}

fn find_copies(book_store: &HashMap<Book, u32>, book: Book) -> Option<&u32> {
    book_store.get(&book)
}
