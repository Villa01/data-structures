mod linked_list;

fn main() {
    let node2 = linked_list::insert_at_beginning(None, 3);
    let node1 = linked_list::insert_at_beginning(Some(node2), 2);
    let mut head = linked_list::insert_at_beginning(Some(node1), 1);

    head = linked_list::insert_at_end(Some(head), 5);
    head = linked_list::insert_at_end(Some(head), 6);

    head = linked_list::insert_at_position(Some(head), 4, 2);
    head = linked_list::insert_at_position(Some(head), 0, 0);

    linked_list::print_summary(&head);

    head = linked_list::delete_first(Some(head)).unwrap();
    linked_list::print_summary(&head);
    
    head = linked_list::delete_last(Some(head)).unwrap();
    linked_list::print_summary(&head);

    head = linked_list::delete_at_position(Some(head), 2).unwrap();
    linked_list::print_summary(&head);
}
