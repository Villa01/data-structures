use std::fmt::Display;

pub struct ListNode<T: Display> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Display> ListNode<T> {
    pub fn new(value: T) -> ListNode<T> {
        ListNode {
            data: value,
            next: None,
        }
    }

    fn _to_string_debug(&self) -> String {
        format!("[{:p}]", self)
    }

    fn to_string(&self) -> String {
        format!("[{}]", self.data)
    }
}

pub fn print<T: Display>(head: &ListNode<T>) {
    let mut cur = Some(head);

    while let Some(node) = cur {
        print!("{} -> ", node.to_string());
        cur = node.next.as_deref();
    }
    println!("None");
}

pub fn print_summary<T: Display>(head: &ListNode<T>) {
    print(&head);
    println!("Linked list length {len}", len = length(&head));
}

pub fn length<T: Display>(head: &ListNode<T>) -> i32 {
    let mut cur = Some(head);
    let mut count = 0;
    while let Some(node) = cur {
        count += 1;
        cur = node.next.as_deref();
    }
    count
}

// Receives an option because the List could be empty
pub fn insert_at_beginning<T: Display>(
    head: Option<Box<ListNode<T>>>,
    data: T,
) -> Box<ListNode<T>> {
    let mut new_node = Box::new(ListNode::new(data));

    match head {
        Some(node) => {
            new_node.next = Some(node);
            return new_node;
        }
        None => new_node,
    }
}

pub fn insert_at_end<T: Display>(mut head: Option<Box<ListNode<T>>>, data: T) -> Box<ListNode<T>> {
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

pub fn insert_at_position<T: Display>(
    mut head: Option<Box<ListNode<T>>>,
    data: T,
    pos: usize,
) -> Box<ListNode<T>> {
    if pos == 0 {
        return insert_at_beginning(head, data);
    }

    let mut cur = &mut head;
    let mut counter = 0;

    while let Some(ref mut node) = *cur {
        if counter == pos - 1 {
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
pub fn delete_first<T: Display>(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
    match head {
        Some(first_node) => return first_node.next,
        None => None,
    }
}

pub fn delete_last<T: Display>(mut head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
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

pub fn delete_at_position<T: Display>(
    mut head: Option<Box<ListNode<T>>>,
    position: usize,
) -> Option<Box<ListNode<T>>> {
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
                if counter >= position - 1 {
                    break;
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
        println!("Test");
        let head = insert_at_beginning::<i32>(None, 1);
        let string = head.to_string();
        assert!(!string.is_empty());
        assert_eq!(string, "[1]");
        assert_ne!(string, "[2]");
    }

    #[test]
    fn test_print() {
        let head = insert_at_beginning::<i32>(None, 1);
        print(&head);
        print_summary(&head);
    }

    #[test]
    fn test_length() {
        let mut head = insert_at_beginning::<i32>(None, 1);
        head = insert_at_beginning(Some(head), 1);
        head = insert_at_beginning(Some(head), 1);

        assert_eq!(length(&head), 3);
    }

    #[test]
    fn test_linked_list_to_string_debug() {
        let head = insert_at_beginning::<i32>(None, 1);
        let string = head._to_string_debug();
        assert!(!string.is_empty());
        assert_ne!(string, "[2]");
    }

    #[test]
    fn test_insert_at_beggining() {
        let head = insert_at_beginning::<i32>(None, 1);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());
    }

    #[test]
    fn test_insert_at_incorrect() {
        let head = insert_at_beginning::<i32>(None, 1);
        assert_ne!((*head).data, 10);
        assert!(!(*head).next.is_some());
    }

    #[test]
    fn test_insert_at_beggining_existing_list() {
        let mut head = insert_at_beginning::<i32>(None, 2);
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
        let mut head = insert_at_end::<i32>(None, 1);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());
        head = insert_at_end(Some(head), 3);
        head = insert_at_end(Some(head), 2);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_some());
    }

    #[test]
    fn test_insert_at_position() {
        let mut head = insert_at_position::<i32>(None, 1, 0);
        assert_eq!((*head).data, 1);
        assert!((*head).next.is_none());

        let list_length = 29;

        for i in 0..list_length - 2 {
            head = insert_at_position(Some(head), i, 0);
        }

        let position = 15;
        let value = 69;
        head = insert_at_position(Some(head), 69, position);

        let mut counter = 0;
        let mut cur = &head;

        while counter < position {
            if let Some(ref node) = cur.next {
                cur = &node;
                counter += 1;
            }
        }
        print_summary(&head);
        assert_eq!(length(&head), list_length);
        assert_eq!((*cur).data, value);
    }

    #[test]
    #[should_panic]
    fn test_insert_at_wrong_position() {
        insert_at_position::<i32>(None, 1, 10);
    }

    #[test]
    fn test_delete_first() {
        let none = delete_first::<i32>(None);

        assert!(none.is_none());

        let mut head = insert_at_beginning::<i32>(None, 4);
        // Retuns None if there is no more nodes
        head = insert_at_beginning(Some(head), 3);
        head = insert_at_beginning(Some(head), 2);
        head = insert_at_beginning(Some(head), 1);

        let new_head = delete_first::<i32>(Some(head));

        assert!(new_head.is_some());

        match new_head {
            Some(node) => {
                assert_eq!(2, node.data)
            }
            None => panic!("The new head should exist"),
        }
    }

    #[test]
    fn test_delete_last() {
        let none = delete_last::<i32>(None);

        assert!(none.is_none());

        let mut list_length = 100;

        let mut head = insert_at_beginning::<i32>(None, 69);

        for i in 1..list_length {
            head = insert_at_beginning(Some(head), i);
        }
        print_summary(&head);

        head = delete_last(Some(head)).unwrap();
        list_length -= 1;

        let mut cur = &head;

        while let Some(ref node) = (*cur).next {
            cur = node;
        }

        assert_eq!(list_length, length(&head));
        assert_eq!(1, (*cur).data);
    }

    #[test]
    fn test_delete_last_single_node() {
        let mut head = Some(insert_at_beginning::<i32>(None, 69));

        head = delete_last(head);

        assert!(head.is_none());
    }

    #[test]
    fn test_delete_at_position() {
        let list_length = 5;
        let mut head = insert_at_beginning(None, list_length);

        for i in 1..list_length {
            head = insert_at_beginning(Some(head), list_length - i);
        }

        let delete_position = 2;
        let mut counter = 0;

        let original_data: i32;

        {
            let mut original_cur = &head;
            while counter < delete_position {
                if let Some(ref node) = original_cur.next {
                    original_cur = &node;
                    counter += 1;
                }
            }
            original_data = (*original_cur).data;
        }

        head = delete_at_position(Some(head), delete_position).unwrap();

        let mut counter = 0;
        let mut cur = &head;

        while counter < delete_position {
            if let Some(ref node) = cur.next {
                cur = &node;
                counter += 1;
            }
        }
        assert_ne!(original_data, (*cur).data);
    }

    #[test]
    fn delete_at_position_fist_item() {
        let mut head = insert_at_beginning(None, 1);
        head = insert_at_beginning(Some(head), 0);

        head = delete_at_position(Some(head), 0).unwrap();

        assert_eq!(1, head.data);
    }

    #[test]
    fn delete_at_position_out_of_bounds() {
        let list_length = 5;
        let mut head = insert_at_beginning::<i32>(None, list_length);

        for i in 1..list_length {
            head = insert_at_beginning(Some(head), list_length - i);
        }

        head = delete_at_position(Some(head), (list_length + 1000) as usize).unwrap();

        // Should have not deleted something
        assert_eq!(length(&head), list_length);
    }

    #[test]
    fn delete_at_position_none_head() {
        assert!(delete_at_position::<i32>(None, 100).is_none())
    }
}
