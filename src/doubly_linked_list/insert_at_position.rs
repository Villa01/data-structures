use super::{DoublyLinkedList, ListNode};
impl<T> DoublyLinkedList<T> {
    /// Inserts a new element at the specified position in the `DoublyLinkedList`.
    ///
    /// # Parameters
    ///
    /// - `data`: The data of type `T` to be inserted into the list.
    /// - `pos`: The zero-based index at which the new element should be inserted.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the element is successfully inserted at the specified position.
    /// - `Err(&str)`: If the position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// list.insert_at_position(42, 0).unwrap(); // Insert at the beginning
    /// list.insert_at_position(84, 1).unwrap(); // Insert at the end
    /// list.insert_at_position(63, 1).unwrap(); // Insert in the middle
    ///
    /// assert_eq!(list.get_first(), Some(&42));
    ///
    /// assert!(list.insert_at_position(99, 10).is_err()); // Out of bounds
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error string `"Index out of bounds"` if `pos > self.len()`.
    ///
    /// # Notes
    ///
    /// - This method uses `insert_at_position_from_start` for positions in the first half of the list
    ///   and `insert_at_position_from_end` for positions in the second half for efficiency.
    /// - Ensure that `pos` is within valid bounds to avoid a panic.
    ///
    /// # Complexity
    ///
    /// - **Best Case (Insert at Beginning/End):** `O(1)`
    /// - **Average Case (Insert at Position):** `O(n/2)`   
    pub fn insert_at_position(&mut self, data: T, pos: usize) -> Result<(), &str> {
        if pos > self.len() {
            return Err("Index out of bounds");
        }

        // Insert at beginning and insert at end are more efficient
        if pos == 0 {
            self.insert_at_beginning(data);
            return Ok({});
        }

        if pos == self.len() {
            self.insert_at_end(data);
            return Ok({});
        }

        if pos < (self.len() / 2) {
            self.insert_at_position_from_start(data, pos);
            return Ok({});
        } else {
            self.insert_at_position_from_end(data, pos);
            return Ok({});
        }
    }

    fn insert_at_position_from_start(&mut self, data: T, pos: usize) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        let mut cursor = self.head;
        let mut counter = 0;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    // Link the new node with the previous node
                    if let Some(n) = (*node).prev {
                        (*n).next = Some(new_node);
                        (*new_node).prev = Some(n);
                    }
                    // Link the new node with the next node
                    (*new_node).next = Some(node);
                    (*node).prev = Some(new_node);

                    self.length += 1;
                    return;
                }

                cursor = (*node).next;
                counter += 1;
            }
        }
    }

    fn insert_at_position_from_end(&mut self, data: T, pos: usize) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        let mut cursor = self.tail;
        let mut counter = self.len() - 1;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    // Link the new node with the previous node
                    if let Some(n) = (*node).prev {
                        (*n).next = Some(new_node);
                        (*new_node).prev = Some(n);
                    }
                    // Link the new node with the next node
                    (*new_node).next = Some(node);
                    (*node).prev = Some(new_node);

                    self.length += 1;
                    return;
                }

                cursor = (*node).prev;
                counter -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at_zero() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let _ = list.insert_at_position(1, 0);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_at_non_zero_out_of_bounds() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let _ = list.insert_at_position(1, 20).unwrap();
    }

    #[test]
    fn test_existing_head() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(2);
        list.insert_at_position(1, 1).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_with_several_nodes_from_start() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        list.insert_at_position(69, 3).unwrap();

        assert_eq!(list.len(), list_length + 1);
    }

    #[test]
    fn test_with_several_nodes_from_end() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        list.insert_at_position(69, 7).unwrap();

        assert_eq!(list.len(), list_length + 1);
    }
}
