use super::DoublyLinkedList;
impl<T> DoublyLinkedList<T> {
    /// Retrieves a mutable reference to the element at the specified position in the `DoublyLinkedList`.
    ///
    /// # Parameters
    ///
    /// - `pos`: The zero-based index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(&mut T)`: A mutable reference to the element at the specified position, if it exists.
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
    /// assert_eq!(list.get_mut(0), Some(&mut 10)); // Get the first element
    /// assert_eq!(list.get_mut(1), Some(&mut 20)); // Get the second element
    /// assert_eq!(list.get_mut(2), Some(&mut 30)); // Get the last element
    /// ```
    pub fn get_mut(&self, pos: usize) -> Option<&mut T> {
        assert!(pos < self.len());

        // Get first and last are more efficient
        if pos == 0 {
            return self.get_first_mut();
        }

        if pos == self.len() - 1 {
            return self.get_last_mut();
        }

        if pos < (self.len() / 2) {
            return self.get_from_start_mut(pos);
        } else {
            return self.get_from_end_mut(pos);
        }
    }

    fn get_from_start_mut(&self, pos: usize) -> Option<&mut T> {
        let mut cursor = self.head;
        let mut counter = 0;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    return Some(&mut (*node).data);
                }

                cursor = (*node).next;
                counter += 1;
            }
        }
        unreachable!()
    }

    fn get_from_end_mut(&self, pos: usize) -> Option<&mut T> {
        let mut cursor = self.tail;
        let mut counter = self.len() - 1;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    return Some(&mut (*node).data);
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
    use super::*;

    #[test]
    fn test_at_zero() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut value = 69;
        let _ = list.insert_at_beginning(value);
        let first = list.get_mut(0).unwrap();
        assert_eq!(&mut value, first);
    }

    #[test]
    fn test_at_len() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut value = 69;
        list.insert_at_end(10);
        list.insert_at_end(value);
        let first = list.get_mut(1).unwrap();
        assert_eq!(&mut value, first);
    }

    #[test]
    #[should_panic]
    fn test_at_non_zero_out_of_bounds() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let _ = list.get_mut(20).unwrap();
    }

    #[test]
    fn test_with_several_nodes_from_start() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        let pos = list.len() / 2 - 1;
        let mut value = 69;
        list.insert_at_position(value, pos).unwrap();

        let get_value = list.get_mut(pos).unwrap();

        assert_eq!(&mut value, get_value);
    }

    #[test]
    fn test_with_several_nodes_from_end() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

        let list_length: usize = 10;

        for i in 1..list_length + 1 {
            list.insert_at_position(i, 0).unwrap();
        }

        let pos = list.len() / 2;
        let mut value = 69;
        list.insert_at_position(value, pos).unwrap();

        let get_value = list.get_mut(pos).unwrap();

        assert_eq!(&mut value, get_value);
    }
}

