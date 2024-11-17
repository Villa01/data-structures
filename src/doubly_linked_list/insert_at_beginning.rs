use super::{DoublyLinkedList, ListNode};

impl<T> DoublyLinkedList<T> {
    /// Inserts an item of type `T` at the start of the `DoublyLinkedList`
    ///
    /// # Parameters
    /// - `data`: The new value to be inserted.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// list.insert_at_beginning(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn insert_at_beginning(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));
        match self.head {
            Some(head) => {
                unsafe {
                    (*new_node).next = Some(head);
                    (*head).prev = Some(new_node);
                }
                self.head = Some(new_node);
            }
            None => {
                // The list is empty and should be initialized

                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.insert_at_beginning(1);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_existing_head() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(2);
        list.insert_at_beginning(1);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_with_several_nodes() {
        struct T {}
        let mut list: DoublyLinkedList<T> = DoublyLinkedList::new();

        let list_length: usize = 100;

        for _ in 1..list_length + 1 {
            list.insert_at_beginning(T {});
        }

        assert_eq!(list.len(), list_length);
    }
}
