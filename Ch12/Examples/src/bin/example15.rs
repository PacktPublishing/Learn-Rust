use std::rc::Rc;
use std::cell::Cell;

struct SharedState {
    immutable_field: String,
    mutable_field: Cell<u32>,
}

fn main() {
    let shared_state = Rc::new(SharedState {
        immutable_field: String::from("Shared State"),
        mutable_field: Cell::new(42),
    });

    let owner1 = Rc::clone(&shared_state);
    let owner2 = Rc::clone(&shared_state);

    println!("Owner1: Immutable: {}, Mutable: {}", owner1.immutable_field, owner1.mutable_field.get());

    owner2.mutable_field.set(84);
    println!("Owner2 updated the mutable field.");

    println!("Owner1: Immutable: {}, Mutable: {}", owner1.immutable_field, owner1.mutable_field.get());
    println!("Owner2: Immutable: {}, Mutable: {}", owner2.immutable_field, owner2.mutable_field.get());
}
