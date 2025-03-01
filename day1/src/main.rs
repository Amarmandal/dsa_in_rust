mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    list.push_back(3);
    list.print();
    list.insert_at(2, 1);
    list.print();
    list.insert_at(0, 0);
    list.print();
    list.insert_at(4, 10);
    list.print();
}
