mod linked_list;

fn main() {
    let mut list_node = linked_list::ListNode::new(1);
    let list_node2 = linked_list::ListNode::new(2);
    let list_node3 = linked_list::ListNode::new(3);
    list_node.next = Some(Box::new(list_node2)); // list_node2 is moved here
    list_node.next.as_mut().unwrap().next = Some(Box::new(list_node3)); // Add list_node3

    linked_list::print(&list_node);
}
