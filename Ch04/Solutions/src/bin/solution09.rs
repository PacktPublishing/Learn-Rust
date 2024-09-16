struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    fn get_first(&self) -> &T {
        &self.first
    }

    fn set_first(&mut self, new_first: T) {
        self.first = new_first;
    }

    fn get_second(&self) -> &U {
        &self.second
    }

    fn set_second(&mut self, new_second: U) {
        self.second = new_second;
    }
}

fn main() {
    let mut pair1 = Pair::new(42, String::from("One"));
    println!("First value: {}", pair1.get_first());

    pair1.set_first(100);
    println!("New first value: {}", pair1.get_first());

    println!("Second value: {}", pair1.get_second());

    pair1.set_second(String::from("Two"));
    println!("New Second value: {}", pair1.get_second());
}
