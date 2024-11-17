use super::DoublyLinkedList;

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
