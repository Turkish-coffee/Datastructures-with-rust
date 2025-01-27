use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Clone> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

// Implement the From trait to convert Node to Option<Rc<RefCell<Node<T>>>>
impl<T: Clone> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

pub type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct List<T: Clone> {
    pub head: Option<NodePtr<T>>,
    pub tail: Option<NodePtr<T>>,
    pub length: usize,
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

// Drop implementation to avoid memory leaks
impl<T: Clone> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}
