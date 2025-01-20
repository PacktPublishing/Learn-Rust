trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog: Box<dyn Animal> = Box::new(Dog);
    dog.speak();
}
