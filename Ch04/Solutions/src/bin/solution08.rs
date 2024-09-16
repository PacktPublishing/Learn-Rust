struct Counter {
    count: i32,
}

impl Counter {
    fn reset(&mut self) {
        println!("Resetting counter from {} to 0", self.count);
        self.count = 0;
    }

    fn get_count(&self) -> i32 {
        println!("Current count is: {}", self.count);
        self.count
    }

    fn increment(&mut self) {
        self.count += 1;
        println!("Incremented count to: {}", self.count);
    }
}

fn main() {
    let mut counter = Counter { count: 0 };

    counter.increment();
    counter.increment();

    let current_count = counter.get_count();
    println!("{}", current_count);

    counter.reset();

    println!("{}", counter.get_count());
}
