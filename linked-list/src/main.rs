mod doubly_linked_list;
mod linked_list;

use doubly_linked_list::DoublyLinkedList;
// use linked_list::LinkedList;

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(200);
    list.push_back(2111);

    list.print();

    list.pop();

    list.print();
}
