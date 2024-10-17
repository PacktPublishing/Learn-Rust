use std::collections::BTreeMap;

fn main() {
    let mut students: BTreeMap<String, i32> = BTreeMap::new();
    students.insert("Eve".to_string(), 88);
    students.insert("Bob".to_string(), 70);
    students.insert("Charlie".to_string(), 95);
    students.insert("David".to_string(), 60);
    students.insert("Alice".to_string(), 85);

    print_students_above_threshold(&students, 80);
}

fn print_students_above_threshold(students: &BTreeMap<String, i32>, threshold: i32) {
    for (name, score) in students.iter() {
        if score > &threshold {
            println!("{}: {}", name, score);
        }
    }
}
