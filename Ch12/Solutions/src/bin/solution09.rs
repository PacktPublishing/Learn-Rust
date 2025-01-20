use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
struct MyStruct {
    name: String,
    age: u32,
    mutable_field: Cell<u32>,
}

impl MyStruct {
    fn new(name: &str, age: u32, initial_value: u32) -> Self {
        MyStruct {
            name: name.to_string(),
            age,
            mutable_field: Cell::new(initial_value),
        }
    }

    fn update_mutable_field(&self, new_value: u32) {
        self.mutable_field.set(new_value);
    }
}

fn main() {
    let my_struct = MyStruct::new("Alice", 30, 100);

    println!("Before update:");
    println!("{:#?}", my_struct);

    my_struct.update_mutable_field(200);

    println!("After update:");
    println!("{:#?}", my_struct);
}
