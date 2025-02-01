use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T: Copy + Debug> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Copy + Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Copy + Debug> LinkedList<T> {
    /// Creates a new empty linked list.
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Adds a node to the end of the list.
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None => {
                // The list was empty: new node becomes both head and tail.
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    /// Adds a node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
            prev: None,
        }));

        if let Some(ref next_node) = new_node.borrow().next {
            next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        } else {
            // List was empty so tail also becomes the new node.
            self.tail = Some(Rc::clone(&new_node));
        }
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Removes the node at the front of the list and returns its value.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // Save the value (T is Copy, so we copy it).
            let value = old_head.borrow().value;
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                // The list is now empty.
                self.tail.take();
            }
            self.size -= 1;
            value
        })
    }

    /// Removes the node at the back of the list and returns its value.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            let value = old_tail.borrow().value;
            if let Some(prev_weak) = old_tail.borrow_mut().prev.take() {
                if let Some(prev) = prev_weak.upgrade() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }
            } else {
                // The list becomes empty.
                self.head.take();
            }
            self.size -= 1;
            value
        })
    }

    /// Updates the node at the given 0-based index with a new value.
    pub fn update_at(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }
        let mut current = self.head.as_ref().map(Rc::clone);
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                node.borrow_mut().value = value;
                return Ok(());
            }
            current = node.borrow().next.as_ref().map(Rc::clone);
            i += 1;
        }
        Err("Index not found")
    }

    /// Traverses the list from head to tail, collecting node values.
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = self.head.as_ref().map(Rc::clone);
        while let Some(node) = current {
            vec.push(node.borrow().value);
            current = node.borrow().next.as_ref().map(Rc::clone);
        }
        vec
    }

    /// Returns the current size of the list.
    pub fn len(&self) -> usize {
        self.size
    }
}