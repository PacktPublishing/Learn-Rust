use std::thread;

fn main() {
    let data = String::from("Hello from captured environment!");

    let handle = thread::spawn(move || {
        println!("{}", data);
    });

    handle.join().unwrap();
}
