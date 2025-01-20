use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: std::fmt::Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn add_head(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
            prev: None,
        }));

        if let Some(old_head) = &self.head {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        } else {
            self.tail = Some(Rc::downgrade(&new_node));
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn add_tail(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.take(),
        }));

        if let Some(old_tail) = self.tail.as_ref().and_then(|weak| weak.upgrade()) {
            old_tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(Rc::downgrade(&new_node));
        self.size += 1;
    }

    pub fn traverse(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }

    pub fn get(&self, position: usize) -> Option<T>
    where
        T: Clone,
    {
        if position >= self.size {
            return None;
        }

        let mut current = self.head.clone();
        for _ in 0..position {
            current = current?.borrow().next.clone();
        }
        current.map(|node| node.borrow().value.clone())
    }

    pub fn update(&mut self, position: usize, value: T) -> Result<(), &str> {
        if position >= self.size {
            return Err("Index out of bounds");
        }

        let mut current = self.head.clone();
        for _ in 0..position {
            current = match current {
                Some(node) => node.borrow().next.clone(),
                None => return Err("Position not found in the list"),
            };
        }

        if let Some(node) = current {
            node.borrow_mut().value = value;
            Ok(())
        } else {
            Err("Node not found")
        }
    }

    pub fn remove(&mut self, position: usize) -> Result<(), &str> {
        if position >= self.size {
            return Err("Index out of bounds");
        }

        let mut current = self.head.clone();
        for _ in 0..position {
            current = match current {
                Some(node) => node.borrow().next.clone(),
                None => return Err("Position not found in the list"),
            };
        }

        if let Some(node) = current {
            let prev = node.borrow_mut().prev.take();
            let next = node.borrow_mut().next.take();

            if let Some(prev_node) = prev.clone().and_then(|weak| weak.upgrade()) {
                prev_node.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(next_node) = next {
                next_node.borrow_mut().prev = prev;
            } else {
                self.tail = prev.map(|weak| Rc::downgrade(&weak.upgrade().unwrap()));
            }

            self.size -= 1;
            Ok(())
        } else {
            Err("Node not found")
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }
}
