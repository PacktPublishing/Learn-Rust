#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);

    println!("Person struct: {:?}", person);
}