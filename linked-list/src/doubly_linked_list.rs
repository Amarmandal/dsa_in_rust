use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Node<T> {
    prev: Option<Rc<RefCell<Node<T>>>>,
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList { head: None }
    }

    pub fn push_back(&mut self, data: T) -> T
    where
        T: std::fmt::Display + Clone,
    {
        if self.head.is_none() {
            let new_data = data.clone();

            let new_node = Rc::new(RefCell::new(Node {
                data: new_data,
                prev: None,
                next: None,
            }));

            self.head = Some(new_node);

            return data;
        }

        let mut current = self.head.clone();

        while let Some(node) = current {
            // if its the last node
            if node.borrow().next.is_none() {
                let new_data = data.clone();

                let new_node = Rc::new(RefCell::new(Node {
                    prev: Some(Rc::clone(&node)),
                    data: new_data,
                    next: None,
                }));

                node.borrow_mut().next = Some(new_node);

                return data;
            }

            current = node.borrow().next.clone();
        }

        unreachable!()
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        // handle empty case
        if self.head.is_none() {
            return None;
        }

        if self
            .head
            .as_ref()
            .map_or(false, |n| n.borrow().next.is_none())
        {
            let node = self.head.take().unwrap();
            return Some(node.borrow().data.clone());
        }

        let mut current = self.head.clone();

        while let Some(node) = current {
            // If next is None, we reached the last node
            if node.borrow().next.is_none() {
                let prev_node = node.borrow_mut().prev.take().unwrap();
                prev_node.borrow_mut().next.take();
                return Some(node.borrow().data.clone()); // Return last node
            }

            current = node.borrow().next.clone(); // Move to the next node
        }

        unreachable!()
    }

    pub fn push(&mut self, data: T) -> Option<T>
    where
        T: std::fmt::Display + Clone,
    {
        let cloned_val = data.clone();
        let new_node = Rc::new(RefCell::new(Node {
            prev: None,
            data,
            next: self.head.clone(),
        }));

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
        }

        self.head = Some(new_node);

        Some(cloned_val)
    }

    pub fn traverse_to(&self, target: i32)
    where
        T: std::fmt::Display,
    {
        let mut current = self.head.clone();
        let mut counter = 1;

        while counter < target {
            match current {
                Some(node) => {
                    current = node.borrow().next.clone();
                }
                None => {
                    current = None;
                }
            }

            counter += 1;
        }

        if let Some(node) = current {
            println!("Value of target node is: {}", node.borrow().data);
        }

        println!("None!");
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = self.head.clone();

        while let Some(node) = current {
            print!("{} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }

        println!("None!");
    }
}
