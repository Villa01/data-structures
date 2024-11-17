use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Gets the amount of items in the `DoublyLinkedList`.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    /// let list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(69);
    /// assert_eq!(list.len(),1);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_len() {
        let list_length: usize = 100;
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        for i in 1..list_length + 1 {
            list.insert_at_beginning(i);
        }
        assert_eq!(list.len(), list_length);
    }
}
