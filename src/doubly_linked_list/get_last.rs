use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Retrieves a reference to the last element in the `DoublyLinkedList`, if it exists.
    ///
    /// # Returns
    ///
    /// - `Some(&T)`: A reference to the last element in the list.
    /// - `None`: If the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// assert_eq!(list.get_last(), None);
    ///
    /// list.insert_at_beginning(42);
    /// list.insert_at_beginning(84);
    ///
    /// assert_eq!(list.get_last(), Some(&42));
    /// ```
    /// # Complexity
    /// - `O(1)`
    ///
    pub fn get_last(&self) -> Option<&T> {
        match self.tail {
            Some(tail) => unsafe { return Some(&(*tail).data) },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_last_empty_list() {
        let list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        let i = list.get_last();

        assert!(i.is_none());
    }

    #[test]
    fn test_get_last() {
        let value = 10;
        let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
        let i = list.get_last();

        assert!(i.is_some());
        assert_eq!(value, *(i.unwrap()));
    }
    #[test]
    fn test_get_last_verify_inmutable() {
        let value = 10;
        let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
        let i = list.get_last();

        assert!(i.is_some());
        assert_eq!(value, *(i.unwrap()));
    }
}
