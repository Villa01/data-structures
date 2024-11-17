use super::{DoublyLinkedList, ListNode};
use std::fmt;

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
mod test {

    use super::*;
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
}
