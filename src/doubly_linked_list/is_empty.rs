use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Checks if the `DoublyLinkedList` is empty.
    ///
    /// # Returns
    /// `true` if the list has no elements, and `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let empty_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// assert!(empty_list.is_empty());
    ///
    /// let list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(69);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0 && self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_empty() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        assert!(list.is_empty());
        list.insert_at_beginning(1);
        assert!(!list.is_empty());
    }
}
