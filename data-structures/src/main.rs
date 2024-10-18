mod linked_list;

fn main() {
    let node2 = linked_list::insert_at_beginning(None, 1);
    let node1 = linked_list::insert_at_beginning(Some(node2), 2);
    let mut head = linked_list::insert_at_beginning(Some(node1), 3);

    head = linked_list::insert_at_end(Some(head), 4);
    head = linked_list::insert_at_end(Some(head), 5);

    linked_list::print(&head);
    println!(
        "Linked list length {len}",
        len = linked_list::length(&head)
    );
}
