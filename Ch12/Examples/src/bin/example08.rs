use std::sync::Arc;
use std::thread;

fn main() {
    let shared_value = Arc::new(42);

    let clone1 = shared_value.clone();
    let clone2 = shared_value.clone();

    let handle1 = thread::spawn(move || {
        println!("Thread 1: {}", clone1);
    });

    let handle2 = thread::spawn(move || {
        println!("Thread 2: {}", clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Reference count after threads: {}", Arc::strong_count(&shared_value));
}
