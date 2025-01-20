use std::rc::{Rc, Weak};

fn main() {
    let strong_rc = Rc::new(5);
    let weak_rc: Weak<i32> = Rc::downgrade(&strong_rc);
    println!("Strong count: {}", Rc::strong_count(&strong_rc));
    println!("Weak count: {}", Rc::weak_count(&strong_rc));

    if let Some(upgraded) = weak_rc.upgrade() {
        println!("Upgraded value: {}", upgraded);
    } else {
        println!("Value has been dropped");
    }

    drop(strong_rc);

    if let Some(upgraded) = weak_rc.upgrade() {
        println!("Upgraded value: {}", upgraded);
    } else {
        println!("Value has been dropped");
    }
}
