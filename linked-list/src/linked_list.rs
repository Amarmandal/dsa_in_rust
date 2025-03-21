pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define the LinkedList structure
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a new node to the front of the list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(), // Take ownership of the current head
        });
        self.head = Some(new_node); // Set new node as head
    }

    pub fn pop(&mut self) -> Option<T> {
        let current_head = self.head.take();

        match current_head {
            Some(node) => {
                self.head = node.next;
                return Some(node.data);
            }
            None => {
                return None;
            }
        }
    }

    // fn reverse(&mut self) {
    //     if self.head.is_none() {
    //         println!("Empty List!");
    //         return;
    //     }

    //     if self
    //         .head
    //         .as_ref()
    //         .map_or(false, |n| n.borrow().next.is_none())
    //     {
    //         return;
    //     }

    //     let mut prev: Option<Rc<RefCell<Node>>> = None;
    //     let mut current = self.head.clone();

    //     while let Some(node) = current {
    //         let next = node.borrow().next.as_ref().map(Rc::clone);
    //         node.borrow_mut().next = prev.as_ref().map(Rc::clone);
    //         prev = Some(node.clone());
    //         current = next;
    //     }

    //     self.head = prev;
    // }

    pub fn insert_at(&mut self, data: T, index: usize)
    where
        T: std::fmt::Display,
    {
        let mut counter = 0;

        if index == 0 || self.head.is_none() {
            self.push(data);
            return;
        }

        // condition for at least two nodes
        let mut before_target_node = self.head.as_mut().unwrap();

        while counter < index - 1 {
            if let Some(ref mut next_node) = before_target_node.next {
                before_target_node = next_node;
            }
            counter += 1;
        }

        // at the position before target node
        let new_node = Box::new(Node {
            data,
            next: before_target_node.next.take(),
        });

        before_target_node.next = Some(new_node);
    }

    pub fn delete_at(&mut self, index: usize) -> Option<T> {
        let mut counter = 0;

        if self.head.is_none() {
            return None;
        }

        if index == 0 {
            return self.pop();
        }

        // for 2 or more elements
        let mut before_target_node = self.head.as_mut().unwrap();

        while counter < index - 1 {
            if let Some(ref mut next_node) = before_target_node.next {
                before_target_node = next_node;
            }
            counter += 1;
        }

        if let Some(node_to_delete) = before_target_node.next.take() {
            before_target_node.next = node_to_delete.next;
            return Some(node_to_delete.data);
        }

        None
    }

    pub fn push_back(&mut self, data: T)
    where
        T: std::fmt::Display + Copy,
    {
        let new_node = Box::new(Node { data, next: None });

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut().unwrap();

        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }

        // at this point this is the last none
        current.next = Some(new_node);
    }

    // Print the list (assuming T can be displayed)
    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}
