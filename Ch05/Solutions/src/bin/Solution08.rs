trait Container {
    type Item;

    fn contains(&self, item: &Self::Item) -> bool;
}

struct BoxContainer<T> {
    value: T,
}

impl<T: PartialEq> Container for BoxContainer<T> {
    type Item = T;

    fn contains(&self, item: &T) -> bool {
        &self.value == item
    }
}

fn main() {
    let container = BoxContainer { value: 42 };
    println!("Contains 42? {}", container.contains(&42));
    println!("Contains 99? {}", container.contains(&99));
}
