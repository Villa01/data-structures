/// A binary tree data structure.
///
/// The binary tree organizes data hierarchically, starting from a root node.
/// Each node can have at most two children: left and right, and optionally
/// a pointer to its parent.
///
/// # Type Parameters
/// - `T`: The type of data stored in the tree nodes. This type must implement
///   the `Ord` trait to allow comparisons, enabling insertion and traversal.
///
/// # Fields
/// - `root`: A mutable pointer to the root node of the tree. If the tree is
///   empty, this will be `None`.
///
/// # Safety
/// Since the `root` is a raw pointer (`*mut`), care must be taken when working
/// with the tree to avoid dereferencing null or dangling pointers. Proper
/// memory management is crucial.
///
pub struct BinaryTree<T: std::cmp::Ord> {
    root: Option<*mut Node<T>>,
}

/// A node in the [`BinaryTree`].
///
/// Represents an individual element of the binary tree, storing data and
/// maintaining a reference to its parent node.
///
/// # Type Parameters
/// - `T`: The type of the data stored in the node. This type must implement
///   the `Ord` trait to enable comparisons for tree operations.
///
/// # Fields
/// - `data`: The value stored in the node.
/// - `parent`: An optional mutable pointer to the parent node. This allows
///   navigating upward in the tree hierarchy.
///
/// # Safety
/// The use of raw pointers (`*mut`) for the `parent` field requires careful
/// handling to avoid dereferencing null or dangling pointers. Ensure proper
/// memory management to prevent undefined behavior.
///
struct Node<T: std::cmp::Ord> {
    data: T,
    parent: Option<*mut Node<T>>,
}

pub mod new;
