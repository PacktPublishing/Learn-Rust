trait Animal {
    fn make_sound(&self);
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bark!");
    }
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn main() {}