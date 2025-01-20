use std::rc::Rc;

fn main() {
    let shared_string = Rc::new(String::from("Hello, Rc!"));

    println!("Initial reference count: {}", Rc::strong_count(&shared_string));

    let cloned_string1 = Rc::clone(&shared_string);
    println!("Reference count after first clone: {}", Rc::strong_count(&shared_string));

    let cloned_string2 = Rc::clone(&shared_string);
    println!("Reference count after second clone: {}", Rc::strong_count(&shared_string));

    println!("Shared string: {}", shared_string);
    println!("Cloned string 1: {}", cloned_string1);
    println!("Cloned string 2: {}", cloned_string2);
}
