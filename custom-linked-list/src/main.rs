mod linked_list;

use self::linked_list::*;

fn main() {
    println!("{:-^1$}", "Enum Version", 100);

    let mut linked_list_enum = LinkedListEnum::new();

    linked_list_enum.insert(0, 1);
    linked_list_enum.insert(1, 3);
    linked_list_enum.insert(1, 5);

    println!("{:#?}", linked_list_enum);

    linked_list_enum.remove(1);

    println!("\ndelete from index 1...\n\n{:#?}", linked_list_enum);
    println!("{:-^1$}", "Struct Version", 100);

    let mut linked_list_struct = LinkedListStruct::new();

    linked_list_struct.insert(1, 1);
    linked_list_struct.insert(1, 3);
    linked_list_struct.insert(0, 5);

    println!("{:#?}", linked_list_struct);

    linked_list_struct.remove(0);

    println!("\ndelete from index 0...\n\n{:#?}", linked_list_struct);
}
