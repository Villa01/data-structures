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

pub mod delete;
pub mod delete_first;
pub mod delete_last;
pub mod display;
pub mod drop;
pub mod get;
pub mod get_first;
pub mod get_first_mut;
pub mod get_last;
pub mod get_last_mut;
pub mod get_mut;
pub mod insert_at_beginning;
pub mod insert_at_end;
pub mod insert_at_position;
pub mod is_empty;
pub mod len;
pub mod new;
pub mod with_value;
