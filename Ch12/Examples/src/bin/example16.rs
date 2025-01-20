use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let shared_data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..3 {
        let shared_data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = shared_data.write().unwrap();
            data.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Data: {:?}", shared_data.read().unwrap());
}