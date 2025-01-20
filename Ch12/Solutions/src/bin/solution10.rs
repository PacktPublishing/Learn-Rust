use std::cell::RefCell;

fn main() {
    let shared_data = RefCell::new(42);

    // Create an immutable borrow
    let _immutable_borrow = shared_data.borrow();
    println!("Immutable borrow: {}", _immutable_borrow);

    // This will panic due to a borrowing conflict
    let _mutable_borrow = shared_data.borrow_mut();
    println!("Mutable borrow: {}", _mutable_borrow);
}
