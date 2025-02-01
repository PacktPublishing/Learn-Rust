mod linked_list;

use std::fmt::Debug;
use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    // Create: Add nodes at both the front and back.
    list.push_back(10);
    list.push_back(20);
    list.push_front(5);

    // Read: Traverse and print the list.
    println!("List values: {:?}", list.to_vec());
    println!("List size: {}", list.len());

    // Update: Change the value at index 1.
    if let Err(e) = list.update_at(1, 42) {
        println!("Update failed: {}", e);
    }
    println!("After update: {:?}", list.to_vec());

    // Delete: Remove nodes from the front and back.
    if let Some(value) = list.pop_front() {
        println!("Popped from front: {}", value);
    }
    if let Some(value) = list.pop_back() {
        println!("Popped from back: {}", value);
    }
    println!("Final list: {:?}", list.to_vec());
}