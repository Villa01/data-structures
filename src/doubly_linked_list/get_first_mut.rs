use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Retrieves a reference to the first element in the `DoublyLinkedList`, if it exists.
    ///
    /// # Returns
    ///
    /// - `Some(&mut T)`: A mutable reference to the first element in the list.
    /// - `None`: If the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// assert_eq!(list.get_first_mut(), None);
    ///
    /// list.insert_at_beginning(42);
    /// list.insert_at_beginning(84);
    ///
    /// assert_eq!(list.get_first_mut(), Some(&mut 84));
    /// ```
    /// # Complexity
    /// - `O(1)`
    ///
    pub fn get_first_mut(&self) -> Option<&mut T> {
        match self.head {
            Some(head) => unsafe { return Some(&mut (*head).data) },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_first_empty_list() {
        let list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        let i = list.get_first_mut();

        assert!(i.is_none());
    }

    #[test]
    fn test_get_first() {
        let value = 10;
        let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
        let i = list.get_first_mut();
        assert!(i.is_some());

        let ivalue = i.unwrap();
        assert_eq!(value, *(ivalue));
        (*ivalue) += 1;

        let new_value = list.get_first().unwrap();
        assert_eq!(value + 1, *(new_value));
    }
}
