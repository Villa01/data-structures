pub struct ListNode {
    pub data: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> ListNode {
        ListNode {
            data: value,
            next: None,
        }
    }

    fn _to_string_debug(&self) -> String {
        format!("[{:p} {}]", self, self.data)
    }

    pub fn to_string(&self) -> String {
        format!("[{}]", self.data)
    }
}

pub fn print(head: &ListNode) {
    let mut cur = Some(head);

    while let Some(node) = cur {
        print!("{} -> ", node.to_string());
        cur = node.next.as_deref();
    }
    println!("None");
}

pub fn print_summary(head: &ListNode) {
    print(&head);
    println!("Linked list length {len}", len = length(&head));
}

pub fn length(head: &ListNode) -> i32 {
    let mut cur = Some(head);
    let mut count = 0;
    while let Some(node) = cur {
        count += 1;
        cur = node.next.as_deref();
    }
    count
}

// Receives an option because the List could be empty
pub fn insert_at_beginning(head: Option<Box<ListNode>>, data: i32) -> Box<ListNode> {
    let mut new_node = Box::new(ListNode::new(data));

    match head {
        Some(node) => {
            new_node.next = Some(node);
            return new_node;
        }
        None => new_node,
    }
}

pub fn insert_at_end(mut head: Option<Box<ListNode>>, data: i32) -> Box<ListNode> {
    let new_node = Box::new(ListNode::new(data));

    match head {
        Some(ref mut node) => {
            let mut cur = node;
            while let Some(ref mut temp) = cur.next {
                cur = temp;
            }
            cur.next = Some(new_node);
            return head.unwrap();
        }
        None => new_node,
    }
}

pub fn insert_at_position(mut head: Option<Box<ListNode>>, data: i32, pos: usize) -> Box<ListNode> {
    if pos == 0 {
        return insert_at_beginning(head, data);
    }

    let mut cur = &mut head;
    let mut counter = 0;

    while let Some(ref mut node) = *cur {
        // TODO: Check issue when inserting
        if counter == pos {
            let mut new_node = ListNode::new(data);
            new_node.next = node.next.take();
            node.next = Some(Box::new(new_node));

            return head.expect("head node must exist to insert at position");
        }

        counter += 1;
        cur = &mut (*node).next;
    }

    return head.expect("head node must exist to insert at position");
}

// Returns the new head
pub fn delete_first(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(first_node) => return first_node.next,
        None => None,
    }
}

pub fn delete_last(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(ref mut node) => {
            let mut cur = node;

            // Iterate until we find the second to last
            while let Some(ref mut current) = (*cur).next {
                let next = &current.next;
                // 1 -> 2 -> 3 -> None
                //      ^    ^
                //   current |
                //          next
                if let Some(next) = next {
                    if next.next.is_none() {
                        (*current).next = None;
                        return head;
                    }
                }
                cur = current;
            }
            // next node does not exist, so we delete the head.
            return None;
        }
        None => None,
    }
}

pub fn delete_at_position(
    mut head: Option<Box<ListNode>>,
    position: usize,
) -> Option<Box<ListNode>> {
    if position == 0 {
        return delete_first(head);
    }

    match head {
        Some(ref mut node) => {
            let mut cur = node;
            let mut counter = 1;

            // Iterate until we find the position
            while let Some(ref mut current) = (*cur).next {
                // i = 2
                // 1 -> 2 -> 3 -> 4 -> None
                //      ^    ^  ^
                //   current |  |
                //       remove |
                //             next
                if counter == position - 1 {
                    if let Some(ref mut next) = current.next {
                        (*current).next = (*next).next.take();
                    }
                    return head;
                }
                cur = current;
                counter += 1;
            }

            return head;
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list_to_string() {
        let head = insert_at_beginning(None, 1);
        let string = head.to_string();
        assert!(!string.is_empty());
        assert_eq!(string, "[1]");
        assert_ne!(string, "[2]");
    }

    #[test]
    fn test_print() {
        let head = insert_at_beginning(None, 1);
        print(&head);
        print_summary(&head);
    }

    #[test]
    fn test_length() {
        let mut head = insert_at_beginning(None, 1);
        head = insert_at_beginning(Some(head), 1);
        head = insert_at_beginning(Some(head), 1);

        assert_eq!(length(&head), 3);
    }

    #[test]
    fn test_linked_list_to_string_debug() {
        let head = insert_at_beginning(None, 1);
        let string = head._to_string_debug();
        assert!(!string.is_empty());
        assert_ne!(string, "[2]");
    }

    #[test]
    fn test_insert_at_beggining() {
        let head = insert_at_beginning(None, 1);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());
    }

    #[test]
    fn test_insert_at_incorrect() {
        let head = insert_at_beginning(None, 1);
        assert_ne!((*head).data, 10);
        assert!(!(*head).next.is_some());
    }

    #[test]
    fn test_insert_at_beggining_existing_list() {
        let mut head = insert_at_beginning(None, 2);
        head = insert_at_beginning(Some(head), 1);
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

    #[test]
    fn test_insert_at_end() {
        let mut head = insert_at_end(None, 1);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());
        head = insert_at_end(Some(head), 3);
        head = insert_at_end(Some(head), 2);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_some());
    }

    #[test]
    fn test_insert_at_position() {
        let mut head = insert_at_position(None, 1, 0);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());

        head = insert_at_position(Some(head), 4, 2);
        head = insert_at_position(Some(head), 0, 0);
        print_summary(&head);
        assert_eq!(length(&head), 3);
        assert!((*head).next.is_some());
    }

    #[test]
    #[should_panic]
    fn test_insert_at_wrong_position() {
        insert_at_position(None, 1, 10);
    }
}
