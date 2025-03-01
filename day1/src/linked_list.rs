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

    pub fn pop(&mut self) {
        let current_head = self.head.take();

        match current_head {
            Some(node) => self.head = node.next,
            None => println!("Nothing to pop!"),
        }
    }

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
