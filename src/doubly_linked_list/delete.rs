use super::DoublyLinkedList;

impl<T: std::fmt::Display> DoublyLinkedList<T> {
    pub fn delete(&mut self, pos: usize) {
        assert!(pos < self.len(), "index out of bounds");

        if pos == 0 {
            self.delete_first();
            return;
        }

        if pos == self.len() - 1 {
            self.delete_last();
            return;
        }

        if pos >= (self.len() / 2) {
            self.delete_from_end(pos);
            return;
        }
        self.delete_from_start(pos);
    }

    fn delete_from_start(&mut self, pos: usize) {
        let mut cursor = self.head;
        let mut counter = 0;
        while let Some(node) = cursor {
            unsafe {
                if pos == counter {
                    if let (Some(prev), Some(next)) = ((*node).prev, (*node).next) {
                        (*prev).next = Some(next);
                        (*next).prev = Some(prev);

                        let _ = Box::from_raw(node);
                        self.length -= 1;
                        return;
                    }
                }

                cursor = (*node).next;
                counter += 1;
            }
        }
        unreachable!()
    }

    fn delete_from_end(&mut self, pos: usize) {
        let mut cursor = self.tail;
        let mut counter = self.len() - 1;
        while let Some(node) = cursor {
            unsafe {
                if pos == counter {
                    if let (Some(prev), Some(next)) = ((*node).prev, (*node).next) {
                        (*prev).next = Some(next);
                        (*next).prev = Some(prev);

                        let _ = Box::from_raw(node);
                        self.length -= 1;
                        return;
                    }
                }

                cursor = (*node).prev;
                counter -= 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::doubly_linked_list::DoublyLinkedList;

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        list.insert_at_beginning(5);
        list.insert_at_beginning(6);
        list.insert_at_beginning(7);
        list.delete(7);
    }

    #[test]
    fn in_of_bounds() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        list.insert_at_beginning(5);
        list.insert_at_beginning(6);
        list.insert_at_beginning(7);
        list.delete(6);
        list.delete(1);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn length_should_decrease() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        list.insert_at_beginning(5);

        list.delete(3);
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn delete_first_element() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        list.delete(0);
        assert_eq!(list.get(0), Some(&3));
    }

    #[test]
    fn delete_last_element() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        list.delete(3);
        assert_eq!(list.get(2), Some(&2));
    }

    #[test]
    fn delete_one_middle_element() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(1);
        list.insert_at_beginning(2);
        list.insert_at_beginning(3);
        list.insert_at_beginning(4);
        let pos = 1;
        list.delete(pos);
        assert_eq!(list.get(pos), Some(&2));
    }

    #[test]
    fn delete_multiple_elements() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        let list_length = 8;
        for i in 1..=list_length {
            list.insert_at_end(i);
        }
        // Delete from start
        list.delete(1);
        // Delete from end
        list.delete(list.len() - 2);
        // Delete from start
        list.delete(1);
        // Delete from end
        list.delete(list.len() - 2);

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&4));
        assert_eq!(list.get(2), Some(&5));
        assert_eq!(list.get(3), Some(&8));
        assert_eq!(list.len(), (list_length - 4) as usize);
    }
}
