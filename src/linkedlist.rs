// implement the linked list data structure
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Copy> {
    data: T,
    //Option is used in the outer scope for null pointer optimization
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}
// implement the From trait to convert Node to Option<Rc<RefCell<Node<T>>>> easier
impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}
type NodePtr<T> = Rc<RefCell<Node<T>>>;
pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
    length: usize,
}
// push and pop front implementation
impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn push_front(&mut self, data : T){
        let mut node = Node::new(data);
        match &mut self.head.take(){
            None => {
                self.head = node.into();
                self.tail = self.tail.clone();
            }
            Some(current_head) => {
                node.next = Some(current_head.clone());
                self.head = node.into();
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(h));
                }
            }
        }
        self.length += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }
                self.length -= 1;
                return Some(head.data);
            }
        }
    }
}

impl <T: Copy> List<T> {
    pub fn push_back(&mut self, data: T) {
        let mut node = Node::new(data);
        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
        self.length += 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                };
                self.length -= 1;
                return Some(tail.data);
            }
        }
    }
}


impl <T: Copy + PartialEq> List<T> {
    pub fn pop_by_key(&mut self, key : T){
        let mut current = self.head.clone();
        while let Some(node) = current.clone() {
            if node.borrow().data == key {
                let next = node.borrow().next.clone();
                let prev = node.borrow().prev.clone();
                match prev.as_ref() {
                    None => {
                        self.head = next.clone();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade().unwrap();
                        prev.borrow_mut().next = next.clone();
                    }
                }
                match next {
                    None => {
                        if prev.is_none() {
                            self.head = None;
                            self.tail = None;
                        } else {
                            self.tail = prev.and_then(|p| p.upgrade());
                        }
                    }
                    Some(next) => {
                        next.borrow_mut().prev = prev;
                    }
                }
                
                self.length -= 1;
                return;
            }
            current = current.unwrap().borrow().next.clone();
        }
        println!("Not such key in the list");
    }

    pub fn pop_by_position(&mut self, position : u32){
        if position > self.length as u32 {
            println!("Position out of range");
            return;
        }
    }
} 


// since we are dealing with RefCell, we need
// to implement Drop trait to avoid memory leaks.
impl <T : Copy> Drop for List<T>{
    fn drop(&mut self){
        while let Some(_) = self.pop_back() {} 
    }
}

// used to have a visual representation of the list
impl<T: Copy + std::fmt::Debug> List<T> {
    pub fn print(&self) {
        let mut current = self.head.clone();
        print!("List {:p} (len: {}) -> ", self.head.as_ref().map_or(std::ptr::null(), |h| Rc::as_ptr(h)), self.length);
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_back() {
        let mut list = List::new();
        list.print();
        list.push_back(1);
        list.print();
        list.push_back(2);
        list.print();
        list.push_back(3);
        list.print();
        assert_eq!(list.pop_back(), Some(3));
        list.print();
        assert_eq!(list.pop_back(), Some(2));
        list.print();
        assert_eq!(list.pop_back(), Some(1));
        list.print();
        assert_eq!(list.pop_back(), None);
        list.print();
    }
    #[test]
    fn test_push_pop_front() {
        let mut list = List::new();
        list.print();
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();

        assert_eq!(list.pop_front(), Some(3));
        list.print();
        assert_eq!(list.pop_front(), Some(2));
        list.print();
        assert_eq!(list.pop_front(), Some(1));
        list.print();
        assert_eq!(list.pop_front(), None);
        list.print();
    }

    #[test]
    fn pop_by_key_index() {
        let mut list = List::new();
        list.pop_by_key(1);
        list.print();  
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();  
        list.push_front(4);
        list.print();
        list.push_front(5);
        list.print();
        list.pop_by_key(5);
        list.print();
        list.pop_by_key(1);
        list.print();
        list.pop_by_key(3);
        list.print();
        list.pop_by_key(12345);
        list.print();
    }
}