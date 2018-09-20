mod linked_list;

use self::linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::new();

    linked_list.insert(0, 1);
    linked_list.insert(1, 3);
    linked_list.insert(2, 5);
//
    println!("{:#?}", linked_list);

    linked_list.remove(1);

    println!("\ndelete from index 1...\n\n{:#?}", linked_list);
}
