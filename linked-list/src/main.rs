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

    println!("Deleting the node at pos 2");
    list.delete_at(2);

    list.reverse();

    list.print();
}
