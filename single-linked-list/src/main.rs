// A simple program for single linked list

use single_linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.add_at_beginning(5);
    linked_list.add_at_beginning(10);
    linked_list.add_at_beginning(15);
    println!("{}", &linked_list);

    while let Some(val) = linked_list.next() {
        println!("{}", val);
    }

    println!("{}", &linked_list);

    for val in linked_list {
        println!("{val}");
    }
}
