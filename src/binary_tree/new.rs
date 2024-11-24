use super::{BinaryTree, Node};

impl<T: std::cmp::Ord> BinaryTree<T> {
    /// Creates a new, empty binary tree.
    ///
    /// # Returns
    /// A new instance of [`BinaryTree`] with no nodes.
    pub fn new() -> BinaryTree<T> {
        return BinaryTree { root: None };
    }
}

impl<T: std::cmp::Ord> Node<T> {
    /// Creates a new node with the given data and optional parent reference.
    ///
    /// # Arguments
    /// - `data`: The value to store in the node.
    /// - `parent`: An optional mutable pointer to the parent node. Use `None` for root nodes.
    ///
    /// # Returns
    /// A new instance of [`Node`] with the specified data and parent reference.
    ///
    /// # Safety
    /// Ensure that the `parent` pointer, if provided, is valid and points to an existing [`Node`].
    ///
    /// ```rust
    pub fn new(data: T, parent: Option<*mut Node<T>>) -> Node<T> {
        Node { data, parent }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryTree, Node};

    #[test]
    fn test_binary_tree_new() {
        // Test creating a new binary tree
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(
            tree.root.is_none(),
            "Newly created binary tree should have no root"
        );
    }

    #[test]
    fn test_node_new_without_parent() {
        // Test creating a new node without a parent
        let node = Node::new(42, None);
        assert_eq!(node.data, 42, "Node data should be 42");
        assert!(node.parent.is_none(), "Node parent should be None");
    }

    #[test]
    fn test_node_new_with_parent() {
        // Test creating a new node with a parent
        let parent = Box::into_raw(Box::new(Node::new(100, None))); // Create parent node
        let child = Node::new(42, Some(parent));

        unsafe {
            assert_eq!((*parent).data, 100, "Parent node data should be 100");
        }
        assert_eq!(child.data, 42, "Child node data should be 42");
        assert!(child.parent.is_some(), "Child node parent should be Some");
    }
}
