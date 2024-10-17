use std::collections::HashMap;

struct MultiSet<T> {
    elements: HashMap<T, usize>,
}

impl<T: std::cmp::Eq + std::hash::Hash> MultiSet<T> {
    fn new() -> Self {
        MultiSet {
            elements: HashMap::new(),
        }
    }

    fn add(&mut self, element: T) {
        let count = self.elements.entry(element).or_insert(0);
        *count += 1;
    }

    fn remove(&mut self, element: &T) {
        if let Some(count) = self.elements.get_mut(element) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.elements.remove(element);
            }
        }
    }

    fn count(&self, element: &T) -> usize {
        *self.elements.get(element).unwrap_or(&0)
    }
}

impl<T: Eq + std::hash::Hash + std::fmt::Debug> MultiSet<T> {
    fn print(&self) {
        for (element, count) in &self.elements {
            println!("{:?}: {}", element, count);
        }
    }
}

fn main() {
    let mut multiset = MultiSet::new();

    multiset.add("apple");
    multiset.add("apple");
    multiset.add("banana");
    multiset.add("orange");
    multiset.add("banana");

    println!("MultiSet after additions:");
    multiset.print();

    println!("Count of 'apple': {}", multiset.count(&"apple"));  // Output: 2
    println!("Count of 'banana': {}", multiset.count(&"banana"));  // Output: 2
    println!("Count of 'grape': {}", multiset.count(&"grape"));  // Output: 0

    multiset.remove(&"apple");
    multiset.remove(&"banana");

    println!("\nMultiSet after removals:");
    multiset.print();

    multiset.remove(&"apple");
    multiset.remove(&"banana");
    multiset.remove(&"orange");

    println!("\nMultiSet after further removals:");
    multiset.print();
}
