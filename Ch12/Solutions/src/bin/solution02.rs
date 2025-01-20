enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn traverse_and_print(&self) {
        match self {
            List::Cons(value, next) => {
                print!("{} ", value);
                next.traverse_and_print();
            }
            List::Nil => (),
        }
    }
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("Elements in the linked list:");
    list.traverse_and_print();
}
