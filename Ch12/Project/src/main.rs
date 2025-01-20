mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    println!("Adding elements to the list...");
    list.add_head(1);
    list.add_tail(2);
    list.add_tail(3);

    println!("Linked List:");
    list.traverse();

    println!("Updating the second element...");
    list.update(1, 10).unwrap();
    list.traverse();

    println!("Removing the third element...");
    list.remove(2).unwrap();
    list.traverse();

    println!("Getting the first element: {:?}", list.get(0));

    println!("Clearing the list...");
    list.clear();
    println!("After clearing:");
    list.traverse();
}
