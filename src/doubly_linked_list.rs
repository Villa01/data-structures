pub struct DoublyLinkedList<T> {
    pub head: Option<Box<ListNode<T>>>,
    pub length: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new empty `DoublyLinkedList`.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    /// struct T;
    /// let list:DoublyLinkedList<T> = DoublyLinkedList::new();
    /// assert!(list.is_empty());
    /// ````
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            length: 0,
        }
    }

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
    /// assert_eq!(list.length, 1);
    /// ```
    pub fn with_value(data: T) -> DoublyLinkedList<T> {
        let new_node = Box::new(ListNode::new(data));
        DoublyLinkedList {
            head: Some(new_node),
            length: 1,
        }
    }

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
        self.len() == 0
    }

    /// Gets the amount of items in the `DoublyLinkedList`.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(69);
    /// assert_eq!(list.len(),1);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }
}

pub struct ListNode<T> {
    next: Option<Box<ListNode<T>>>,
    pub data: T,
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> ListNode<T> {
        ListNode { data, next: None }
    }
}

#[cfg(test)]
mod tests {

    mod doubly_linked_list {
        use crate::doubly_linked_list::DoublyLinkedList;

        #[test]
        fn test_new() {
            let list: DoublyLinkedList<bool> = DoublyLinkedList::new();
            assert!(list.is_empty())
        }

        #[test]
        fn test_with_value() {
            let list: DoublyLinkedList<bool> = DoublyLinkedList::with_value(true);
            assert!(!list.is_empty())
        }
    }
}
