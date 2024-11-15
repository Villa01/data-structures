/// A doubly-linked list implementation that allows efficient insertion and removal of elements.
///
/// The `DoublyLinkedList` provides methods to insert, remove, and check elements at both ends of the list.
/// It supports constant-time operations for inserting and removing elements at the beginning and end,
/// making it suitable for use cases that require a flexible and efficient collection.
///
/// # Examples
///
/// ```
/// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
///
/// // Create a new, empty list
/// let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
/// assert!(list.is_empty());
///
/// ```
///
/// # Type Parameters
///
/// - `T`: The type of elements stored in the list.
///
/// # Fields
///
/// - `head`: A pointer to the first node in the list. This is `None` when the list is empty.
/// - `length`: The current number of elements in the list.
pub struct DoublyLinkedList<T> {
    head: Option<*mut ListNode<T>>,
    tail: Option<*mut ListNode<T>>,
    length: usize,
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
        let mut new_node = ListNode::new(data);

        DoublyLinkedList {
            head: Some(&mut new_node),
            tail: Some(&mut new_node),
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
        self.len() == 0 && self.head.is_none()
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

    /// Inserts an item of type `T` at the start of the `DoublyLinkedList`
    ///
    /// # Parameters
    /// - `data`: The new value to be inserted.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// list.insert_at_beginning(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn insert_at_beginning(&mut self, data: T) {
        let mut new_node = ListNode::new(data);
        match self.head {
            Some(current_head) => {
                new_node.next = Some(current_head);
                unsafe {
                    (*current_head).prev = Some(&mut new_node);
                }
                self.head = Some(&mut new_node);
            }
            None => {
                // The list is empty and should be initialized

                self.head = Some(&mut new_node);
                self.tail = Some(&mut new_node);
            }
        }
        self.length += 1;
    }
}

/// A node in a doubly linked list.
///
/// Each `ListNode` stores a piece of data of type `T` along with raw pointers to the
/// next and previous nodes in the list. The pointers are wrapped in `Option`
/// to indicate whether the node has a next or previous node.
///
/// # Type Parameters
/// - `T`: The type of the data stored in the node.
///
/// # Fields
/// - `next`: An optional raw pointer to the next node in the list. `None` if there is no next node.
/// - `prev`: An optional raw pointer to the previous node in the list. `None` if there is no previous node.
/// - `data`: The value stored in the node.
///
/// # Safety
/// Since this struct uses raw pointers, care must be taken to ensure that
/// pointers are valid and properly managed. Improper use may result in undefined behavior.
struct ListNode<T> {
    next: Option<*mut ListNode<T>>,
    prev: Option<*mut ListNode<T>>,
    data: T,
}

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

        mod insert_at_beginning {
            use super::*;

            #[test]
            fn test_empty_list() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
                list.insert_at_beginning(1);
                assert!(!list.is_empty());
                assert_eq!(list.len(), 1);
            }

            #[test]
            fn test_existing_head() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(2);
                list.insert_at_beginning(1);
                assert_eq!(list.len(), 2);
            }
        }
    }
}
