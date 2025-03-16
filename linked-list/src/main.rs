mod doubly_linked_list;
// mod linked_list;

// use linked_list::LinkedList;
use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

    list.push(10);

    list.print();

    list.pop();

    list.print();
}
