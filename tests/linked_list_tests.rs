use data_structures::linked_list;

#[test]
fn test_insert_at_beggining() {
    let head = linked_list::insert_at_beginning(None, 1);
    assert_eq!((*head).data, 1);
    assert!((*head).next.is_none());
}

#[test]
fn test_insert_at_incorrect() {
    let head = linked_list::insert_at_beginning(None, 1);
    assert_ne!((*head).data, 10);
    assert!(!(*head).next.is_some());
}

#[test]
fn test_insert_at_beggining_existing_list() {
    let mut head = linked_list::insert_at_beginning(None, 2);
    head = linked_list::insert_at_beginning(Some(head), 1); 
    // Data is correct
    assert_eq!((*head).data, 1);
    // Next node exists
    assert!((*head).next.is_some());
    // Next node's data is correct
    let next_node = head.next.unwrap();
    assert_eq!(next_node.data, 2);    
    // Next node points to None
    assert!(next_node.next.is_none());
}



