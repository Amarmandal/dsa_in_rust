use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[allow(unused)]
impl<T> Node<T>
where
    T: Default,
{
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn new_default() -> Self {
        Node {
            data: T::default(),
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

#[allow(unused)]
impl<T> LinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn remove_nth(&mut self, n: &mut u16) {
        if self.head.is_none() {
            println!("Nothing to remove");
            return;
        }

        let dummy_node = Rc::new(RefCell::new(Node::new_default()));
        dummy_node.borrow_mut().next = self.head.clone();

        let mut left_node = Some(Rc::clone(&dummy_node));
        let mut right_node = self.head.clone();

        while *n > 0 {
            if let Some(current) = right_node.take() {
                if let Some(next_node) = current.borrow().next.clone() {
                    right_node = Some(next_node);
                    *n -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        while let Some(node) = right_node {
            right_node = node.borrow().next.clone();
            left_node = left_node.unwrap().borrow_mut().next.clone();
        }

        left_node.unwrap().borrow_mut().next = left_node
            .as_ref()
            .and_then(|n| n.borrow().next.clone())
            .as_ref()
            .and_then(|n| n.borrow().next.clone());

        dummy_node.borrow_mut().next.take();
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

    pub fn has_cycle(&self) -> bool
    where
        T: std::fmt::Debug,
    {
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
            fast = rabbit
                .borrow()
                .next
                .as_ref()
                .and_then(|n| n.borrow().next.clone());
        }

        false
    }
}
