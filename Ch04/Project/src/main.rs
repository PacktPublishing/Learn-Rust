use std::cmp::PartialEq;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    genre: Genre,
    pages: u32,
    status: BookStatus,
}

impl Book {
    fn new(title: String, author: String, genre: Genre, pages: u32) -> Self {
        Self {
            title,
            author,
            genre,
            pages,
            status: BookStatus::Available, // New books start as available
        }
    }

    fn print_details(&self) {
        println!(
            "[{}, {}, {:?}, Pages: {}, {:?}]",
            self.title, self.author, self.genre, self.pages, self.status
        );
    }

    fn borrow(&mut self) {
        if self.status.is_available() {
            self.status = BookStatus::Borrowed;
            println!("You have borrowed the book: {}", self.title);
        } else {
            println!("The book: {} is not available for borrowing.", self.title);
        }
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
        println!("You have returned the book: {}", self.title);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum BookStatus {
    Available,
    Borrowed,
    Reserved,
}

impl BookStatus {
    fn is_available(&self) -> bool {
        matches!(self, BookStatus::Available)
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Genre {
    Fiction,
    NonFiction,
    Science,
    ScienceFiction,
    History,
    Fantasy,
}

struct Library {
    books: [Book; 3],
}

impl Library {
    fn new(books: [Book; 3]) -> Self {
        Self { books }
    }

    fn find_book_by_title_as_mut(&mut self, title: &str) -> Option<&mut Book> {
        for book in &mut self.books {
            if book.title == title {
                return Some(book);
            }
        }
        None
    }

    fn list_available_books(&self) {
        println!("Available Books:");
        for book in &self.books {
            if book.status.is_available() {
                book.print_details();
            }
        }
    }

    fn borrow_book(&mut self, title: &str) {
        match self.find_book_by_title_as_mut(title) {
            None => {println!("Book not found: {}", title);}
            Some(book) => {book.borrow()}
        }
    }

    fn return_book(&mut self, title: &str) {
        match self.find_book_by_title_as_mut(title) {
            None => {println!("Book not found: {}", title);}
            Some(book) => {book.return_book()}
        }
    }

    fn list_books_by_genre(&self, genre: Genre) {
        println!("Books in the genre: {:?}", genre);
        for book in &self.books {
            if genre == book.genre {
                book.print_details();
            }
        }
    }
}

fn main() {
    // Create a new library
    let mut my_library = Library::new(
        [
            Book::new(
                String::from("Starlight Chronicles: Voyage Beyond the Edge of Time"),
                String::from("Dr. Celeste Orion"),
                Genre::ScienceFiction,
                500),
            Book::new(
                String::from("The Hobbit"),
                String::from("J.R.R. Tolkien"),
                Genre::Fantasy,
                310),
            Book::new(
                String::from("A Brief History of Time"),
                String::from("Stephen Hawking"),
                Genre::Science,
                256),
        ]
    );

    // List available books
    my_library.list_available_books();

    // Borrow a book
    my_library.borrow_book("The Hobbit");

    // List available books after borrowing one
    my_library.list_available_books();

    // Return a book
    my_library.return_book("The Hobbit");

    // List available books after borrowing one
    my_library.list_available_books();

    // Borrow another book
    my_library.borrow_book("The Rust Programming Language");

    // List books by genre
    my_library.list_books_by_genre(Genre::Fantasy);
}