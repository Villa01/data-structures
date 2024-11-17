use super::{DoublyLinkedList, ListNode};

impl<T> DoublyLinkedList<T> {
    /// Creates a new `DoublyLinkedList` initialized with a single element.
    ///
    /// # Parameters
    ///
    /// - `data`: The initial value for the first node in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    /// let list = DoublyLinkedList::with_value(42);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn with_value(data: T) -> DoublyLinkedList<T> {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        DoublyLinkedList {
            head: Some(new_node),
            tail: Some(new_node),
            length: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_value() {
        let list: DoublyLinkedList<bool> = DoublyLinkedList::with_value(true);
        assert!(!list.is_empty());
    }
}
