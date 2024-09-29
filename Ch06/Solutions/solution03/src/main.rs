fn main() {
    school::students::add_student();
    school::classes::add_class();
}

mod school {
    pub mod students {
        pub fn add_student() {
            println!("Student added to the school.");
        }
    }

    pub mod classes {
        pub fn add_class() {
            println!("Class added to the school.");
        }
    }
}
