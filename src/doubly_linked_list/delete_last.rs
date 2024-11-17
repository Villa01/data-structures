use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Removes the last element from the `DoublyLinkedList`, if it exists.
    ///
    /// # Behavior
    ///
    /// - If the list is empty, this method does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// list.insert_at_end(1);
    /// list.insert_at_end(2);
    /// list.insert_at_end(3);
    ///
    /// list.delete_last();
    /// assert_eq!(list.get(1), Some(&2)); // The new head is now 2
    /// assert_eq!(list.len(), 2);         // The length of the list is updated
    ///
    /// list.delete_last();
    /// list.delete_last();
    /// assert!(list.is_empty());          // The list is empty after all elements are removed
    /// ```
    /// # Safety
    ///
    /// - This method uses `unsafe` internally to deallocate the removed node from the heap.
    /// - It is safe as long as the internal structure of the `DoublyLinkedList` remains valid,
    ///   ensuring proper pointer updates between nodes.
    ///
    /// # Complexity
    ///
    /// - **Time Complexity:** `O(1)`
    /// - **Space Complexity:** `O(1)`
    pub fn delete_last(&mut self) {
        if let Some(tail) = self.tail {
            unsafe {
                // One item lists
                if self.head == self.tail {
                    // Deallocate memory in heap
                    self.head = None;
                    self.tail = None;
                } else if let Some(prev) = (*tail).prev {
                    self.tail = Some(prev);
                    (*prev).next = None;
                }
                let _ = Box::from_raw(tail);
            }
            self.length -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_delete_last_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.delete_last();
        assert!(
            list.is_empty(),
            "List should remain empty after deleting from an empty list"
        );
    }

    #[test]
    fn test_delete_last_one_item_list() {
        let mut list = DoublyLinkedList::new();
        list.insert_at_end(42);

        list.delete_last();
        assert!(
            list.is_empty(),
            "List should be empty after deleting the only item"
        );
        assert_eq!(list.len(), 0, "List length should be 0 after deletion");
    }

    #[test]
    fn test_delete_first_multi_item_list() {
        let mut list = DoublyLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        list.delete_last();
        assert_eq!(
            list.get(1),
            Some(&2),
            "Last element should now be 2 after deleting the first"
        );
        assert_eq!(list.len(), 2, "List length should be 2 after deletion");

        list.delete_last();
        assert_eq!(
            list.get(0),
            Some(&1),
            "Last element should now be 3 after another deletion"
        );
        assert_eq!(
            list.len(),
            1,
            "List length should be 1 after another deletion"
        );

        list.delete_last();
        assert!(
            list.is_empty(),
            "List should be empty after deleting all elements"
        );
        assert_eq!(
            list.len(),
            0,
            "List length should be 0 after deleting all elements"
        );
    }

    #[test]
    fn test_delete_until_empty() {
        let mut list = DoublyLinkedList::new();
        list.insert_at_end(10);
        list.insert_at_end(20);
        list.insert_at_end(30);

        list.delete_last();
        list.delete_last();
        list.delete_last();

        assert!(
            list.is_empty(),
            "List should be empty after deleting all elements"
        );
        assert_eq!(
            list.len(),
            0,
            "List length should be 0 after deleting all elements"
        );
        list.delete_first();
    }

    #[test]
    fn test_list_integrity_after_delete_last() {
        let mut list = DoublyLinkedList::new();
        list.insert_at_end(5);
        list.insert_at_end(10);
        list.insert_at_end(15);

        list.delete_last();
        list.insert_at_end(20);

        assert_eq!(
            list.get(list.len() - 1),
            Some(&20),
            "Last element should now be 20"
        );
        assert_eq!(
            list.get(list.len() - 2),
            Some(&10),
            "Second element should remain 10"
        );
        assert_eq!(
            list.get(list.len() - 3),
            Some(&5),
            "Newly inserted element should be at the end"
        );
        assert_eq!(list.len(), 3, "List length should be 3 after operations");
    }
}
