use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    {
        let value = data.borrow();
        println!("Immutable borrow: {}", value);
    }

    {
        let mut value_mut = data.borrow_mut();
        *value_mut += 10;
    }

    println!("New value after mutation: {}", data.borrow());
}