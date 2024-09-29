mod library {
    pub mod books {
        pub fn add_book() {
            println!("A new book has been added.");
        }

        fn remove_book() {
            println!("A book has been removed.");
        }
    }

    pub mod users {
        pub fn add_user() {
            println!("A new user has been added.");
        }
    }
}

fn main() {
    library::books::add_book();
    library::users::add_user();
}
