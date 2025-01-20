use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Node::new(1);

    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());

    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("Parent of child1 is: {}", parent.value);
    } else {
        println!("Child1 has no parent");
    }

    println!("Root value: {}", root.value);
    for child in root.children.borrow().iter() {
        println!("Child value: {}", child.value);
    }
}
