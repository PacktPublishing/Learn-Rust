use std::sync::Mutex;

fn main() {
    let m = Mutex::new(17);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("Updated value: {:?}", m.lock().unwrap());
}
