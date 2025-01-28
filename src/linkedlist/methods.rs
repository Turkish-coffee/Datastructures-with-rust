#![allow(dead_code)]

use crate::linkedlist::definitions::{List, Node,};
use std::rc::Rc;

impl<T: Clone> List<T> {
    // Pushes a new element to the front of the list
    pub fn push_front(&mut self, data: T) {
        let mut node = Node::new(data);
        match self.head.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
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
    // Pops the first element from the list
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
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
                Some(head.data.clone())
            }
        }
    }
    // Pushes a new element to the back of the list
    pub fn push_back(&mut self, data: T) {
        let mut node = Node::new(data);
        match self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
        self.length += 1;
    }
    // Pops the last element from the list
    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
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
                Some(tail.data.clone())
            }
        }
    }
}

impl<T: Clone + PartialEq> List<T> {
    // Removes the first occurrence of the element from the list
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
    // Removes the element at the specified position from the list
    pub fn pop_by_position(&mut self, position : usize){
        if position >= self.length {
            println!("Position out of range");
            return;
        }else {
            let mut current = self.head.clone();
            for _ in 0..position {
                current = current.unwrap().borrow().next.clone();
            }
            let next = current.clone().unwrap().borrow().next.clone();
            let prev = current.clone().unwrap().borrow().prev.clone();
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
    }
    // Removes all occurrences of the element from the list
    pub fn update_by_key(&mut self, key : T, new_data : T){
        let mut current = self.head.clone();
        while let Some(node) = current.clone() {
            if node.borrow().data == key {
                node.borrow_mut().data = new_data;
                return;
            }
            current = node.borrow().next.clone();
        }
        println!("Not such key in the list");
    }
    // Updates the element at the specified position with the new data
    pub fn update_by_position(&mut self, position : usize, new_data : T){
        if position >= self.length {
            println!("Position out of range");
            return;
        }else {
            let mut current = self.head.clone();
            for _ in 0..position {
                current = current.unwrap().borrow().next.clone();
            }
            current.unwrap().borrow_mut().data = new_data;
        }
    }
}

impl<T: Clone + std::fmt::Debug> List<T> {
    // Prints the list
    pub fn print(&self) {
        let mut current = self.head.clone();
        print!("List {:p} (len: {}) -> ",
        self.head.as_ref().map_or(std::ptr::null(), |h| Rc::as_ptr(h)),
        self.length);
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

/*
list of all the other function signatures to implement  :
pub fn push_by_position(&mut self,  data : T, position : usize){
    todo!();
}

pub fn count_key_occurences(&self, key : T) -> u32{
    todo!();
}

pub fn  swap_nodes_by_position(&mut self, position1 : usize, position2 : usize){
    todo!();
}

pub fn  swap_nodes_by_key(&mut self, key1 : T, key2 : T){
    todo!();
}

pub fn reverse(&mut self){
    todo!();
}

pub fn remove_duplicates(&mut self){
    todo!();
}

pub fn sort(&mut self){
    todo!();
}

pub fn split_by_key(&mut self, key : T) -> (List<T>, List<T>) {
    todo!();
}

pub fn split_by_position(&mut self, position : usize) -> (List<T>, List<T>){
    todo!();
}

pub fn merge(&mut self, other : List<T>){
    todo!();
}

// returns a linkedlist perfroming union elementwise
pub fn union(&self, other : List<T>) -> List<T>{
    todo!();
}

// returns a linkedlist perfroming intersection elementwise
pub fn intersection(&self, other : List<T>) -> List<T>{
    todo!();
}

pub fn sublist_search(&self, other : List<T>) -> bool{
    todo!();
}

pub fn delete_slice(&mut self, start : usize, end : usize){
    todo!();
}
*/