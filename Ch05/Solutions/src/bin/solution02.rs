trait Named {
    fn name(&self) -> String;
}

trait Age {
    fn age(&self) -> u32;
}

struct Person{
    name: String,
    age: u32,
}

impl Named for Person {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Age for Person {
    fn age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 32,
    };

    println!("{}", person.name());
    println!("{}", person.age());
}
