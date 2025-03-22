mod doubly_linked_list;
mod linked_list;

// use doubly_linked_list::DoublyLinkedList;
use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.print();

    println!("Has cycle: {}", list.has_cycle());
}
