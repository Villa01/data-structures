use core::fmt;

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
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        DoublyLinkedList {
            head: Some(new_node),
            tail: Some(new_node),
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
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));
        match self.head {
            Some(head) => {
                unsafe {
                    (*new_node).next = Some(head);
                    (*head).prev = Some(new_node);
                }
                self.head = Some(new_node);
            }
            None => {
                // The list is empty and should be initialized

                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// Inserts an item of type `T` at the end of the `DoublyLinkedList`
    ///
    /// # Parameters
    /// - `data`: The new value to be inserted.
    ///
    /// # Examples
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// list.insert_at_end(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn insert_at_end(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));
        match self.tail {
            Some(tail) => {
                unsafe {
                    (*tail).next = Some(new_node);
                    (*new_node).prev = Some(tail);
                }
                self.tail = Some(new_node);
            }
            None => {
                // The list is empty and should be initialized

                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// Inserts a new element at the specified position in the `DoublyLinkedList`.
    ///
    /// # Parameters
    ///
    /// - `data`: The data of type `T` to be inserted into the list.
    /// - `pos`: The zero-based index at which the new element should be inserted.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the element is successfully inserted at the specified position.
    /// - `Err(&str)`: If the position is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// list.insert_at_position(42, 0).unwrap(); // Insert at the beginning
    /// list.insert_at_position(84, 1).unwrap(); // Insert at the end
    /// list.insert_at_position(63, 1).unwrap(); // Insert in the middle
    ///
    /// assert_eq!(list.get_first(), Some(&42));
    ///
    /// assert!(list.insert_at_position(99, 10).is_err()); // Out of bounds
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error string `"Index out of bounds"` if `pos > self.len()`.
    ///
    /// # Notes
    ///
    /// - This method uses `insert_at_position_from_start` for positions in the first half of the list
    ///   and `insert_at_position_from_end` for positions in the second half for efficiency.
    /// - Ensure that `pos` is within valid bounds to avoid a panic.
    ///
    /// # Complexity
    ///
    /// - **Best Case (Insert at Beginning/End):** `O(1)`
    /// - **Average Case (Insert at Position):** `O(n/2)`   
    pub fn insert_at_position(&mut self, data: T, pos: usize) -> Result<(), &str> {
        if pos > self.len() {
            return Err("Index out of bounds");
        }

        // Insert at beginning and insert at end are more efficient
        if pos == 0 {
            self.insert_at_beginning(data);
            return Ok({});
        }

        if pos == self.len() {
            self.insert_at_end(data);
            return Ok({});
        }

        if pos < (self.len() / 2) {
            println!("Inserting from start");
            self.insert_at_position_from_start(data, pos);
            return Ok({});
        } else {
            println!("Inserting from end");
            self.insert_at_position_from_end(data, pos);
            return Ok({});
        }
    }

    fn insert_at_position_from_start(&mut self, data: T, pos: usize) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        let mut cursor = self.head;
        let mut counter = 0;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    // Link the new node with the previous node
                    if let Some(n) = (*node).prev {
                        (*n).next = Some(new_node);
                        (*new_node).prev = Some(n);
                    }
                    // Link the new node with the next node
                    (*new_node).next = Some(node);
                    (*node).prev = Some(new_node);

                    self.length += 1;
                    return;
                }

                cursor = (*node).next;
                counter += 1;
            }
        }
    }

    fn insert_at_position_from_end(&mut self, data: T, pos: usize) {
        let new_node = Box::into_raw(Box::new(ListNode::new(data)));

        let mut cursor = self.tail;
        let mut counter = self.len() - 1;

        unsafe {
            while let Some(node) = cursor {
                if counter == pos {
                    // Link the new node with the previous node
                    if let Some(n) = (*node).prev {
                        (*n).next = Some(new_node);
                        (*new_node).prev = Some(n);
                    }
                    // Link the new node with the next node
                    (*new_node).next = Some(node);
                    (*node).prev = Some(new_node);

                    self.length += 1;
                    return;
                }

                cursor = (*node).prev;
                counter -= 1;
            }
        }
    }

    /// Retrieves a reference to the first element in the `DoublyLinkedList`, if it exists.
    ///
    /// # Returns
    ///
    /// - `Some(&T)`: A reference to the first element in the list.
    /// - `None`: If the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// assert_eq!(list.get_first(), None);
    ///
    /// list.insert_at_beginning(42);
    /// list.insert_at_beginning(84);
    ///
    /// assert_eq!(list.get_first(), Some(&84));
    /// ```
    /// # Complexity
    /// - `O(1)`
    ///
    pub fn get_first(&self) -> Option<&T> {
        match self.head {
            Some(head) => unsafe { return Some(&(*head).data) },
            None => None,
        }
    }

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

    /// Retrieves a reference to the last element in the `DoublyLinkedList`, if it exists.
    ///
    /// # Returns
    ///
    /// - `Some(&mut T)`: A mutable reference to the last element in the list.
    /// - `None`: If the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use villa01_data_structures::doubly_linked_list::DoublyLinkedList;
    ///
    /// let mut list = DoublyLinkedList::new();
    /// assert_eq!(list.get_last_mut(), None);
    ///
    /// list.insert_at_beginning(42);
    /// list.insert_at_beginning(84);
    ///
    /// assert_eq!(list.get_last_mut(), Some(&mut 42));
    /// ```
    /// # Complexity
    /// - `O(1)`
    ///
    pub fn get_last_mut(&self) -> Option<&mut T> {
        match self.tail {
            Some(tail) => unsafe { return Some(&mut (*tail).data) },
            None => None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len() == 0 {
            write!(f, "EMPTY")?;
            return Ok({});
        }
        let mut cursor = self.head;
        write!(f, "START -> ")?;
        unsafe {
            while let Some(current) = cursor {
                if cursor != self.head {
                    write!(f, " <=>")?;
                }
                write!(f, " {} ", *current)?;
                cursor = (*current).next
            }
        }
        write!(f, "-> NULL")?;
        Ok({})
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.head;
        while let Some(node) = cursor {
            unsafe {
                // Deallocating memory from heap to avoid memory leakage
                let _ = Box::from_raw(node);
                cursor = (*node).next;
            }
        }
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

impl<T: fmt::Display> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str("[ ");
        result.push_str(&(self.data.to_string()));
        result.push_str(" ]");

        return write!(f, "{}", result);
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
            assert!(!list.is_empty());
        }

        #[test]
        fn test_len() {
            let list_length: usize = 100;
            let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

            for i in 1..list_length + 1 {
                list.insert_at_beginning(i);
            }
            assert_eq!(list.len(), list_length);
        }

        #[test]
        fn test_displaying_list() {
            let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
            assert_eq!(list.to_string(), "EMPTY");
            let list_length = 3;
            for i in 1..list_length + 1 {
                list.insert_at_beginning(i);
            }
            assert_eq!(list.len(), list_length);
            let str_list = list.to_string();
            assert_eq!(str_list, "START ->  [ 3 ]  <=> [ 2 ]  <=> [ 1 ] -> NULL");

            let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
            for i in 1..list_length + 1 {
                list.insert_at_end(i);
            }
            println!("{}", list);
            let str_list = list.to_string();
            assert_eq!(str_list, "START ->  [ 1 ]  <=> [ 2 ]  <=> [ 3 ] -> NULL")
        }

        #[test]
        fn test_is_empty() {
            let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
            assert!(list.is_empty());
            list.insert_at_beginning(1);
            assert!(!list.is_empty());
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

            #[test]
            fn test_with_several_nodes() {
                struct T {}
                let mut list: DoublyLinkedList<T> = DoublyLinkedList::new();

                let list_length: usize = 100;

                for _ in 1..list_length + 1 {
                    list.insert_at_beginning(T {});
                }

                assert_eq!(list.len(), list_length);
            }
        }

        mod insert_at_end {
            use super::*;

            #[test]
            fn test_empty_list() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
                list.insert_at_end(1);
                assert!(!list.is_empty());
                assert_eq!(list.len(), 1);
            }

            #[test]
            fn test_existing_head() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(2);
                list.insert_at_end(1);
                assert_eq!(list.len(), 2);
            }

            #[test]
            fn test_with_several_nodes() {
                struct T {}
                let mut list: DoublyLinkedList<T> = DoublyLinkedList::new();

                let list_length: usize = 100;

                for _ in 1..list_length + 1 {
                    list.insert_at_end(T {});
                }

                assert_eq!(list.len(), list_length);
            }
        }

        mod insert_at_position {
            use super::*;

            #[test]
            fn test_at_zero() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
                let _ = list.insert_at_position(1, 0);
                assert!(!list.is_empty());
                assert_eq!(list.len(), 1);
            }

            #[test]
            #[should_panic]
            fn test_at_non_zero_out_of_bounds() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
                let _ = list.insert_at_position(1, 20).unwrap();
            }

            #[test]
            fn test_existing_head() {
                let mut list: DoublyLinkedList<i32> = DoublyLinkedList::with_value(2);
                list.insert_at_position(1, 1).unwrap();
                assert_eq!(list.len(), 2);
            }

            #[test]
            fn test_with_several_nodes_from_start() {
                let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

                let list_length: usize = 10;

                for i in 1..list_length + 1 {
                    list.insert_at_position(i, 0).unwrap();
                }

                list.insert_at_position(69, 3).unwrap();

                assert_eq!(list.len(), list_length + 1);
            }

            #[test]
            fn test_with_several_nodes_from_end() {
                let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();

                let list_length: usize = 10;

                for i in 1..list_length + 1 {
                    list.insert_at_position(i, 0).unwrap();
                }

                list.insert_at_position(69, 7).unwrap();

                assert_eq!(list.len(), list_length + 1);
            }
        }

        mod get_first {
            use super::*;

            #[test]
            fn test_get_first_empty_list() {
                let list: DoublyLinkedList<usize> = DoublyLinkedList::new();
                let i = list.get_first();

                assert!(i.is_none());
            }

            #[test]
            fn test_get_first() {
                let value = 10;
                let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
                let i = list.get_first();

                assert!(i.is_some());
                assert_eq!(value, *(i.unwrap()));
            }
            #[test]
            fn test_get_first_verify_inmutable() {
                let value = 10;
                let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
                let i = list.get_first();

                assert!(i.is_some());
                assert_eq!(value, *(i.unwrap()));
            }
        }

        mod get_first_mut {
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

        mod get_last {
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

        mod get_last_mut {
            use super::*;

            #[test]
            fn test_get_last_empty_list() {
                let list: DoublyLinkedList<usize> = DoublyLinkedList::new();
                let i = list.get_last_mut();

                assert!(i.is_none());
            }

            #[test]
            fn test_get_last() {
                let value = 10;
                let list: DoublyLinkedList<usize> = DoublyLinkedList::with_value(value);
                let i = list.get_last_mut();
                assert!(i.is_some());

                let ivalue = i.unwrap();
                assert_eq!(value, *(ivalue));
                (*ivalue) += 1;

                let new_value = list.get_last().unwrap();
                assert_eq!(value + 1, *(new_value));
            }
        }
    }
}
