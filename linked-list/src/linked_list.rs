use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(), // Take ownership of current head
        }));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.head.take().map(|node| {
            self.head = node.borrow().next.clone(); // Clone next Rc
            node.borrow().data.clone() // Clone data to return
        })
    }

    pub fn reverse(&mut self) {
        if self.head.is_none() {
            return; // Empty list
        }
        if self
            .head
            .as_ref()
            .map_or(false, |n| n.borrow().next.is_none())
        {
            return; // Single node, no reverse needed
        }

        let mut prev: Option<Rc<RefCell<Node<T>>>> = None;
        let mut current = self.head.clone();

        while let Some(node) = current {
            let next = node.borrow().next.clone();
            node.borrow_mut().next = prev.clone();
            prev = Some(node.clone());
            current = next;
        }

        self.head = prev; // New head is the last node
    }

    pub fn insert_at(&mut self, data: T, pos: usize)
    where
        T: std::fmt::Display,
    {
        let index = pos - 1;
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));

        if index == 0 || self.head.is_none() {
            new_node.borrow_mut().next = self.head.take();
            self.head = Some(new_node);
            return;
        }

        let mut counter = 0;
        let mut current = self.head.as_ref().map(Rc::clone);

        while let Some(node) = current {
            if counter == index - 1 {
                new_node.borrow_mut().next = node.borrow().next.clone();
                node.borrow_mut().next = Some(Rc::clone(&new_node));
                return;
            }
            current = node.borrow().next.as_ref().map(Rc::clone);
            counter += 1;
        }
    }

    pub fn delete_at(&mut self, pos: usize) -> Option<T>
    where
        T: Clone,
    {
        let index = pos - 1;

        // Empty list case
        if self.head.is_none() {
            return None;
        }

        // Delete head (position 1)
        if index == 0 {
            let removed_node = self.head.take();
            if let Some(node) = removed_node {
                self.head = node.borrow().next.clone();
                return Some(node.borrow().data.clone());
            }
            return None;
        }

        // Traverse to find node before deletion point
        let mut current = self.head.as_ref().map(Rc::clone);
        let mut counter = 0;

        while let Some(node) = current {
            if counter == index - 1 {
                // Found node before the one to delete
                let next_node = node.borrow().next.clone();
                match next_node {
                    Some(to_delete) => {
                        // Skip the node to delete by connecting to the next one
                        node.borrow_mut().next = to_delete.borrow().next.clone();
                        return Some(to_delete.borrow().data.clone());
                    }
                    None => return None,
                }
            }
            current = node.borrow().next.as_ref().map(Rc::clone);
            counter += 1;
        }

        None // Index out of bounds
    }

    pub fn push_back(&mut self, data: T)
    where
        T: std::fmt::Display,
    {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().next.is_none() {
                node.borrow_mut().next = Some(new_node);
                return;
            }
            current = node.borrow().next.clone();
        }
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
        println!("None");
    }

    pub fn has_cycle(&self) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut slow = self.head.as_ref().map(Rc::clone);
        let mut fast = self.head.clone().unwrap().borrow().next.clone();

        while let Some(rabbit) = fast {
            if let Some(tortoise) = slow {
                if Rc::ptr_eq(&tortoise, &rabbit) {
                    return true;
                }
                slow = tortoise.borrow().next.as_ref().map(Rc::clone);
            }
            // increase rabbit by twice
            fast = rabbit.borrow().next.clone();
            if fast.is_none() {
                return false;
            } else {
                fast = fast.unwrap().borrow().next.clone();
            }
        }

        false
    }
}
