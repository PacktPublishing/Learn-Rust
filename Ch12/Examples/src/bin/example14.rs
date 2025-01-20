use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(17);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("Read lock 1: {}", r1);
        println!("Read lock 2: {}", r2);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("Write lock: {}", w);
    }

    println!("Updated value: {}", lock.read().unwrap());
}