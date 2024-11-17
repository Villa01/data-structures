use super::DoublyLinkedList;

impl<T> DoublyLinkedList<T> {
    /// Retrieves a reference to the element at the specified position in the `DoublyLinkedList`.
    ///
    /// # Parameters
    ///
    /// - `pos`: The zero-based index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(&T)`: A reference to the element at the specified position, if it exists.
    /// - `None`: If the position is out of bounds or the list is empty.
    ///
    /// # Panics
    ///
    /// This method will panic if `pos > self.len()`. Ensure that the provided position is within bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// list.insert_at_end(10);
    /// list.insert_at_end(20);
    /// list.insert_at_end(30);
    ///
    /// assert_eq!(list.get(0), Some(&10)); // Get the first element
    /// assert_eq!(list.get(1), Some(&20)); // Get the second element
    /// assert_eq!(list.get(2), Some(&30)); // Get the last element
    /// ```
    pub fn get(&self, pos: usize) -> Option<&T> {
        assert!(pos < self.len());

        // Get first and last are more efficient
        if pos == 0 {
            return self.get_first();
        }

        if pos == self.len() - 1 {
            return self.get_last();
        }

        if pos < (self.len() / 2) {
            return self.get_from_start(pos);
        } else {
            return self.get_from_end(pos);
        }
    }

    fn get_from_start(&self, pos: usize) -> Option<&T> {
        let mut cursor = self.head;
        let mut counter = 0;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    return Some(&(*node).data);
                }

                cursor = (*node).next;
                counter += 1;
            }
        }
        return None;
    }

    fn get_from_end(&self, pos: usize) -> Option<&T> {
        let mut cursor = self.tail;
        let mut counter = self.len() - 1;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    return Some(&(*node).data);
                }

                cursor = (*node).prev;
                counter -= 1;
            }
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at_zero() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let value = 69;
        let _ = list.insert_at_beginning(value);
        let first = list.get(0).unwrap();
        assert_eq!(&value, first);
    }

    #[test]
    fn test_at_len() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let value = 69;
        list.insert_at_end(10);
        list.insert_at_end(value);
        let first = list.get(1).unwrap();
        assert_eq!(&value, first);
    }

    #[test]
    #[should_panic]
    fn test_at_non_zero_out_of_bounds() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let _ = list.get(20).unwrap();
    }

    #[test]
    fn test_with_several_nodes_from_start() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        let pos = list.len() / 2 - 1;
        let value = 69;
        list.insert_at_position(value, pos).unwrap();

        let get_value = list.get(pos).unwrap();

        assert_eq!(&value, get_value);
    }

    #[test]
    fn test_with_several_nodes_from_end() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        let pos = list.len() / 2;
        let value = 69;
        list.insert_at_position(value, pos).unwrap();

        let get_value = list.get(pos).unwrap();

        assert_eq!(&value, get_value);
    }
}
