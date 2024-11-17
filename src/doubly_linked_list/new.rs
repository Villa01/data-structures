use super::{DoublyLinkedList, ListNode};

impl<T> ListNode<T> {
    /// Creates a new empty `ListNode`.
    pub fn new(data: T) -> ListNode<T> {
        ListNode {
            data,
            next: None,
            prev: None,
        }
    }
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
            tail: None,
            length: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let list: DoublyLinkedList<bool> = DoublyLinkedList::new();
        assert!(list.is_empty())
    }
}
