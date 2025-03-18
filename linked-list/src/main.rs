mod doubly_linked_list;
// mod linked_list;

// use linked_list::LinkedList;
use doubly_linked_list::DoublyLinkedList;

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
