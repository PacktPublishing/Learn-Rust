use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(Cell::new(0));

    let counter_owner1 = Rc::clone(&counter);
    let counter_owner2 = Rc::clone(&counter);

    counter_owner1.set(counter_owner1.get() + 1);
    println!("Counter updated by owner 1: {}", counter_owner1.get());

    counter_owner2.set(counter_owner2.get() + 2);
    println!("Counter updated by owner 2: {}", counter_owner2.get());

    println!("Final counter value: {}", counter.get());
}
