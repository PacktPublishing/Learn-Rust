use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let shared_value = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    for i in 0..5 {
        let shared_value = Arc::clone(&shared_value);
        let handle = thread::spawn(move || {
            for _ in 0..3 {
                let value = shared_value.read().unwrap();
                println!("Reader {}: Value = {}", i, *value);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    {
        let shared_value = Arc::clone(&shared_value);
        let handle = thread::spawn(move || {
            for _ in 0..3 {
                thread::sleep(Duration::from_millis(50));
                let mut value = shared_value.write().unwrap();
                *value += 1;
                println!("Writer: Updated Value = {}", *value);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Value: {}", *shared_value.read().unwrap());
}
