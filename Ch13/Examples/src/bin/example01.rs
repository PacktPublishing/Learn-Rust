use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hello from the thread! Count: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..=5 {
        println!("Hello from the main thread! Count: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    handle.join().unwrap();
}
