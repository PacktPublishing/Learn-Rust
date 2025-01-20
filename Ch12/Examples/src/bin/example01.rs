use std::ops::Deref;

#[derive(Debug)]
struct MySmartPointer<T> {
    data: T,
}

impl<T> MySmartPointer<T> {
    fn new(data: T) -> Self {
        MySmartPointer { data }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    let my_smart_pointer = MySmartPointer::new(String::from("Hello, Rust!"));

    println!("Length: {}", my_smart_pointer.len());
    println!("Uppercase: {}", my_smart_pointer.to_uppercase());

    println!("MySmartPointer: {:#?}", my_smart_pointer);
    println!("Value: {}", *my_smart_pointer);
}
