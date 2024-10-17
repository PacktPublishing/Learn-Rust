use std::collections::HashMap;

fn main() {
    let mut people: HashMap<&str, i32> = HashMap::new();
    people.insert("Alice", 30);
    people.insert("Bob", 25);
    people.insert("Charlie", 35);
    people.insert("Diana", 28);
    people.insert("Eve", 40);

    let name_to_check = "Bob";
    check_name(&mut people, name_to_check);
}

fn check_name(people: &mut HashMap<&str, i32>, name_to_check: &str) {
    match people.get(name_to_check) {
        Some(age) => println!("{} is {} years old.", name_to_check, age),
        None => println!("{} not found in the map.", name_to_check),
    }
}